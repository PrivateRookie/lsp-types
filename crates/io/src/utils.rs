use bytes::{Buf, BytesMut};
use lsp_ty::{NotificationMessage, OneOf3, RequestMessage, ResponseMessage};

use super::BUF_SIZE;

fn parse_header(headers: &str) -> Result<(String, usize), String> {
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
        return Err(msg.to_string());
    }
    Ok((content_type, content_length))
}

#[derive(Debug, Clone)]
pub struct CodecState {
    pub content_type: String,
    pub read_content_length: usize,
    pub read_buf: [u8; BUF_SIZE],
    pub read_data: BytesMut,
}

impl CodecState {
    pub fn consume_body(
        &mut self,
    ) -> serde_json::Result<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
        let msg = serde_json::from_slice(&self.read_data[..self.read_content_length])?;
        // reset state after read
        self.read_data.advance(self.read_content_length);
        self.read_content_length = 0;
        Ok(msg)
    }

    pub fn parse_header(&mut self, headers: String) -> Result<(), String> {
        let (content_type, content_length) = parse_header(&headers)?;
        self.content_type = content_type;
        self.read_content_length = content_length;
        Ok(())
    }

    fn header_pos(&self) -> Option<usize> {
        self.read_data
            .windows(4)
            .position(|s| s == [b'\r', b'\n', b'\r', b'\n'])
    }

    pub fn try_parse_header(&mut self) -> Option<Result<(), String>> {
        self.header_pos().map(|stop_at| {
            let headers = String::from_utf8(self.read_data[..stop_at].to_vec()).unwrap();
            self.read_data.advance(stop_at + 4);
            self.parse_header(headers)
        })
    }

    pub fn body_ready(&self) -> bool {
        self.read_content_length <= self.read_data.len()
    }
}

impl Default for CodecState {
    fn default() -> Self {
        Self {
            content_type: "application/vscode-jsonrpc; charset=utf-8".to_string(),
            read_content_length: Default::default(),
            read_buf: [0; BUF_SIZE],
            read_data: BytesMut::with_capacity(BUF_SIZE),
        }
    }
}
