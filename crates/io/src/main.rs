use bytes::{Buf, BytesMut};
use clap::Parser;
use lsp_ty::{NotificationMessage, OneOf, RequestMessage, ResponseMessage};
use std::{
    io::{Read, Write},
    net::TcpListener,
};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

pub type IOResult<T> = std::io::Result<T>;

const BUF_SIZE: usize = 1024 * 4;

pub struct ServerCodec<S: Read + Write> {
    stream: S,
    read_content_length: usize,
    content_type: String,
    read_buf: BytesMut,
}

impl<S: Read + Write> ServerCodec<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            read_content_length: 0,
            content_type: "application/vscode-jsonrpc; charset=utf-8".to_string(),
            read_buf: BytesMut::with_capacity(BUF_SIZE),
        }
    }
}

impl<S: Read + Write> ServerCodec<S> {
    fn poll(&mut self) -> IOResult<usize> {
        self.stream.read(&mut self.read_buf)
    }

    fn consume_body(&mut self) -> IOResult<OneOf<RequestMessage, NotificationMessage>> {
        let msg: OneOf<RequestMessage, NotificationMessage> =
            serde_json::from_slice(&self.read_buf[..self.read_content_length])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        // reset state after read
        self.read_buf.advance(self.read_content_length);
        self.read_content_length = 0;
        Ok(msg)
    }

    fn parse_header(&mut self, headers: String) -> IOResult<()> {
        for header in headers.split("\r\n") {
            let mut segs = header.split(":");
            let key = segs.next().unwrap();
            if key == "Content-Length" {
                self.read_content_length = segs.next().unwrap().trim().parse().unwrap();
            } else if key == "Content-Type" {
                self.content_type = segs.next().unwrap().trim().to_string();
            } else {
                tracing::error!("unknown header {}", key);
            }
            if self.read_content_length == 0 {
                let msg = "empty content length or missing Content-Length header";
                tracing::error!(msg);
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, msg));
            }
        }
        Ok(())
    }

    pub fn receive(&mut self) -> IOResult<OneOf<RequestMessage, NotificationMessage>> {
        loop {
            if let Some(stop_at) = self
                .read_buf
                .windows(4)
                .position(|s| s == [b'\r', b'\n', b'\r', b'\n'])
            {
                let headers = String::from_utf8(self.read_buf[..stop_at].to_vec()).unwrap();
                self.read_buf.advance(stop_at + 4);
                self.parse_header(headers)?;
                break;
            } else {
                self.poll()?;
            }
        }

        while self.read_content_length > self.read_buf.len() {
            self.poll()?;
        }

        self.consume_body()
    }
}

impl<S: Read + Write> ServerCodec<S> {
    pub fn send(&mut self, message: OneOf<ResponseMessage, NotificationMessage>) -> IOResult<()> {
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
}
pub struct Server<S: Read + Write> {
    codec: ServerCodec<S>,
    terminated: bool,
}

impl<S: Read + Write> Server<S> {
    fn new(stream: S) -> Self {
        Self {
            codec: ServerCodec::new(stream),
            terminated: false,
        }
    }

    fn receive(&mut self) -> IOResult<()> {
        match self.codec.receive()? {
            OneOf::One(req) => self.on_req(req),
            OneOf::Other(notice) => self.on_notify(notice),
        }
    }

    pub fn on_req(&mut self, req: RequestMessage) -> IOResult<()> {
        tracing::debug!("{:?}", req);
        Ok(())
    }

    pub fn on_notify(&mut self, notice: NotificationMessage) -> IOResult<()> {
        tracing::debug!("{:?}", notice);
        Ok(())
    }

    pub fn resp(&mut self, resp: ResponseMessage) -> IOResult<()> {
        self.codec.send(OneOf::One(resp))
    }

    pub fn notify(&mut self, msg: NotificationMessage) -> IOResult<()> {
        self.codec.send(OneOf::Other(msg))
    }

    pub fn run(&mut self) -> IOResult<()> {
        while !self.terminated {
            self.receive()?;
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// listen host
    #[clap(long, default_value = "127.0.0.1")]
    host: String,
    /// listen port
    #[clap(short, long, default_value = "9999")]
    port: u16,
    /// log level
    #[clap(short, long, default_value = "info")]
    level: Level,
}

fn init_log(level: Level) {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(level)
        .finish()
        .try_init()
        .expect("failed to init logging");
}

fn main() -> IOResult<()> {
    let args = Args::parse();
    init_log(args.level);
    let listener =
        TcpListener::bind(format!("{}:{}", args.host, args.port)).expect("failed to bind addr");
    tracing::info!("listening at {}:{}", args.host, args.port);
    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                let mut server = Server::new(conn);
                server.run()?;
            }
            Err(e) => {
                tracing::error!("accept connection failed {}", e.to_string());
            }
        }
    }
    Ok(())
}
