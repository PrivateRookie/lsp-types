use lsp_ty::{NotificationMessage, OneOf3, RequestMessage, ResponseMessage};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::utils::CodecState;

/// async protocol message reader/writer
pub struct AMessageCodec<S: AsyncRead + AsyncWrite> {
    stream: S,
    state: CodecState,
}

impl<S: AsyncRead + AsyncWrite + Unpin> AMessageCodec<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            state: CodecState::default(),
        }
    }

    /// get mutable ref of underlying stream
    pub fn stream_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    async fn poll(&mut self) -> tokio::io::Result<usize> {
        let state = &mut self.state;
        let count = self.stream.read(&mut state.read_buf).await?;
        state.read_data.extend_from_slice(&state.read_buf[..count]);
        Ok(count)
    }

    pub async fn receive(
        &mut self,
    ) -> tokio::io::Result<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
        loop {
            if let Some(may_ok) = self.state.try_parse_header() {
                may_ok.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                break;
            } else {
                self.poll().await?;
            }
        }

        while !self.state.body_ready() {
            self.poll().await?;
        }

        self.state
            .consume_body()
            .map_err(|e| tokio::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// write message to peer
    pub async fn send(
        &mut self,
        message: OneOf3<RequestMessage, ResponseMessage, NotificationMessage>,
    ) -> tokio::io::Result<()> {
        let json_str = serde_json::to_string(&message)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let data = json_str.as_bytes();
        self.stream
            .write_all(
                format!(
                    "Content-Length: {}\r\nContent-Type: {}\r\n\r\n",
                    data.len(),
                    self.state.content_type
                )
                .as_bytes(),
            )
            .await?;
        self.stream.write_all(data).await?;
        Ok(())
    }

    /// helper function to send request only
    pub async fn send_req(&mut self, message: RequestMessage) -> tokio::io::Result<()> {
        self.send(OneOf3::This(message)).await
    }

    /// helper function to send response only
    pub async fn send_resp(&mut self, message: ResponseMessage) -> tokio::io::Result<()> {
        self.send(OneOf3::Among(message)).await
    }

    /// helper function to send notification only
    pub async fn send_notice(&mut self, message: NotificationMessage) -> tokio::io::Result<()> {
        self.send(OneOf3::Other(message)).await
    }
}
