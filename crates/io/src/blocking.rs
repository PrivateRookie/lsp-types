use lsp_ty::{NotificationMessage, OneOf3, RequestMessage, ResponseMessage};
use std::io::{Read, Write};

type IOResult<T> = std::io::Result<T>;

use crate::utils::CodecState;

/// protocol message reader/writer
pub struct Codec<S: Read + Write> {
    stream: S,
    state: CodecState,
}

impl<S: Read + Write> Codec<S> {
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

    fn poll(&mut self) -> IOResult<usize> {
        let state = &mut self.state;
        let count = self.stream.read(&mut state.read_buf)?;
        if count == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "read eof",
            ));
        }
        state.read_data.extend_from_slice(&state.read_buf[..count]);
        Ok(count)
    }

    fn consume_body(
        &mut self,
    ) -> IOResult<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
        self.state
            .consume_body()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// read message from peer
    ///
    /// for server, most of times coming messages are request or notification,
    /// at some rare case, there maybe a response, see [applyEdit](https://microsoft.github.io/language-server-protocol/specifications/specification-3-17/#workspace_applyEdit)
    pub fn receive(
        &mut self,
    ) -> IOResult<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
        loop {
            if let Some(may_ok) = self.state.try_parse_header() {
                may_ok.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                break;
            } else {
                self.poll()?;
            }
        }

        while !self.state.body_ready() {
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
                self.state.content_type
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

#[cfg(feature = "ws")]
mod ws_codec {
    use std::net::TcpStream;

    use lsp_ty::{NotificationMessage, OneOf3, RequestMessage, ResponseMessage};
    use ws_tool::{
        codec::{default_handshake_handler, WsStringCodec},
        frame::OpCode,
        stream::WsStream,
        ClientBuilder, ServerBuilder,
    };

    use super::IOResult;

    pub struct WsCodec {
        ws: WsStringCodec<WsStream>,
    }

    impl WsCodec {
        pub fn new_client<S: ToString>(addr: S) -> IOResult<Self> {
            let ws = ClientBuilder::new(addr).connect(WsStringCodec::check_fn)?;
            Ok(Self { ws })
        }

        pub fn new_server(stream: TcpStream) -> IOResult<Self> {
            let ws =
                ServerBuilder::accept(stream, default_handshake_handler, WsStringCodec::factory)?;
            Ok(Self { ws })
        }

        pub fn stream_mut(&mut self) -> &mut TcpStream {
            self.ws.stream_mut().stream_mut()
        }

        pub fn receive(
            &mut self,
        ) -> IOResult<OneOf3<RequestMessage, ResponseMessage, NotificationMessage>> {
            let (code, data) = self.ws.receive()?;
            if code == OpCode::Close {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    "peer send close",
                ));
            } else if code == OpCode::Text {
                let msg = serde_json::from_str(&data).map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                })?;
                Ok(msg)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("unknown frame code {:?}", code),
                ))
            }
        }

        pub fn send(
            &mut self,
            message: OneOf3<RequestMessage, ResponseMessage, NotificationMessage>,
        ) -> IOResult<()> {
            let json_str = serde_json::to_string(&message)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            self.ws.send((None, json_str))?;
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
}

#[cfg(feature = "ws")]
pub use ws_codec::WsCodec;
