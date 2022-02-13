use clap::Parser;
use lsp_io::ServerCodec;
use lsp_ty::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, Empty, FromReq,
    InitializeParams, InitializeResult, InitializeResultServerInfo, NotificationMessage, OneOf,
    RequestMessage, ResponseMessage, ServerCapabilities, ShutdownParams,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

pub type IOResult<T> = std::io::Result<T>;

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
        InitializeParams::on_req(req, self, |server, id, _| {
            let ret = InitializeResult {
                capabilities: ServerCapabilities {
                    completion_provider: Some(CompletionOptions {
                        all_commit_characters: None,
                        resolve_provider: None,
                        trigger_characters: Some(vec!["$".to_string()]),
                        work_done_progress: None,
                    }),
                    ..Default::default()
                },
                server_info: Some(InitializeResultServerInfo {
                    name: "yaya-server".to_string(),
                    version: Some("0.0.1".to_string()),
                }),
            };
            server.resp(id.ok_resp(ret))
        })
        .map_or(|req| {
            CompletionParams::on_req(req, self, |server, id, _| {
                let ret = CompletionItem {
                    label: "demo".to_string(),
                    detail: Some("that's ok".to_string()),
                    insert_text: Some("yaya".to_string()),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                };
                server.resp(id.ok_resp(vec![ret]))
            })
        })
        .flat_or_map_or(|req| {
            ShutdownParams::on_req(req, self, |server, id, _| {
                server.terminated = true;
                tracing::info!("shutting down...");
                server.resp(id.ok_resp(Empty {}))
            })
        })
        .flat_or()
        .unify(|req| {
            tracing::warn!("unhandled request {:#?}", req);
            self.resp(req.id.ok_resp(serde_json::Value::Null))
        })
    }

    pub fn on_notify(&mut self, notice: NotificationMessage) -> IOResult<()> {
        tracing::info!("{:#?}", notice);
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
                tracing::info!("launching new lsp server...");
                server.run()?;
            }
            Err(e) => {
                tracing::error!("accept connection failed {}", e.to_string());
            }
        }
    }
    Ok(())
}
