use bytes::{Buf, BytesMut};
use lsp_ty::{NotificationMessage, OneOf3, RequestMessage, ResponseMessage};
use std::io::{Read, Write};

pub type IOResult<T> = std::io::Result<T>;

const BUF_SIZE: usize = 1024 * 4;

fn parse_header(headers: &str) -> IOResult<(String, usize)> {
    let mut segs = headers.split(':');
    let key = segs.next().unwrap();
    let mut content_type = String::new();
    let mut content_length = 0;
    if key == "Content-Length" {
        content_length = segs.next().unwrap().trim().parse().unwrap();
    } else if key == "Content-Type" {
        content_type = segs.next().unwrap().trim().to_string();
    } else {
        tracing::error!("unknown header {}", key);
    }
    if content_length == 0 {
        let msg = "empty content length or missing Content-Length header";
        tracing::error!(msg);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, msg));
    }
    Ok((content_type, content_length))
}

/// protocol message encode/decode
pub struct MessageCodec<S: Read + Write> {
    stream: S,
    content_type: String,
    read_content_length: usize,
    read_buf: [u8; BUF_SIZE],
    read_data: BytesMut,
}

impl<S: Read + Write> MessageCodec<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            read_content_length: 0,
            content_type: "application/vscode-jsonrpc; charset=utf-8".to_string(),
            read_data: BytesMut::with_capacity(BUF_SIZE),
            read_buf: [0; BUF_SIZE],
        }
    }

    fn poll(&mut self) -> IOResult<usize> {
        let count = self.stream.read(&mut self.read_buf)?;
        self.read_data.extend_from_slice(&self.read_buf[..count]);
        Ok(count)
    }

    fn consume_body(
        &mut self,
    ) -> IOResult<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
        let msg = serde_json::from_slice(&self.read_data[..self.read_content_length])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        // reset state after read
        self.read_data.advance(self.read_content_length);
        self.read_content_length = 0;
        Ok(msg)
    }

    fn parse_header(&mut self, headers: String) -> IOResult<()> {
        let (content_type, content_length) = parse_header(&headers)?;
        self.content_type = content_type;
        self.read_content_length = content_length;
        Ok(())
    }

    /// read message from peer
    ///
    /// for server, most of times coming messages are request or notification,
    /// at some rare case, there maybe a response, see [applyEdit](https://microsoft.github.io/language-server-protocol/specifications/specification-3-17/#workspace_applyEdit)
    pub fn receive(
        &mut self,
    ) -> IOResult<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
        loop {
            if let Some(stop_at) = self
                .read_data
                .windows(4)
                .position(|s| s == [b'\r', b'\n', b'\r', b'\n'])
            {
                let headers = String::from_utf8(self.read_data[..stop_at].to_vec()).unwrap();
                self.read_data.advance(stop_at + 4);
                self.parse_header(headers)?;
                break;
            } else {
                self.poll()?;
            }
        }

        while self.read_content_length > self.read_data.len() {
            self.poll()?;
        }

        self.consume_body()
    }

    /// write message to peer
    pub fn send(
        &mut self,
        message: OneOf3<RequestMessage, ResponseMessage, NotificationMessage>,
    ) -> IOResult<()> {
        let json_str = serde_json::to_string(&message)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let data = json_str.as_bytes();
        self.stream.write_all(
            format!(
                "Content-Length: {}\r\nContent-Type: {}\r\n\r\n",
                data.len(),
                self.content_type
            )
            .as_bytes(),
        )?;
        self.stream.write_all(data)?;
        Ok(())
    }

    /// helper function to send request only
    pub fn send_req(&mut self, message: RequestMessage) -> IOResult<()> {
        self.send(OneOf3::This(message))
    }

    /// helper function to send response only
    pub fn send_resp(&mut self, message: ResponseMessage) -> IOResult<()> {
        self.send(OneOf3::Among(message))
    }

    /// helper function to send notification only
    pub fn send_notice(&mut self, message: NotificationMessage) -> IOResult<()> {
        self.send(OneOf3::Other(message))
    }
}
