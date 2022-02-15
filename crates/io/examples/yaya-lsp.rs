use clap::Parser;
use lsp_io::ServerCodec;
use lsp_ty::{
    CancelParams, CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, Empty,
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
            OneOf::This(req) => self.on_req(req),
            OneOf::Other(notice) => self.on_notify(notice),
        }
    }

    pub fn on_req(&mut self, req: RequestMessage) -> IOResult<()> {
        // pass context when handle request need;
        req.with(self)
            // pass handler function, you must specify param type
            // in anonymous handler function argument, other wise
            // you have to use turbo fish symbol
            .then(|ctx, id, _: InitializeParams| {
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
                ctx.resp(id.ok_resp(ret))
            })
            // use or_else to route to other handler function if
            // method do not match
            .or_else(|ctx, id, _: CompletionParams| {
                let item = CompletionItem {
                    label: "demo".to_string(),
                    detail: Some("that's ok".to_string()),
                    insert_text: Some("yaya".to_string()),
                    kind: Some(CompletionItemKind::Keyword),
                    ..Default::default()
                };
                ctx.resp(id.ok_resp(vec![item]))
            })
            .or_else(|ctx, id, _: ShutdownParams| {
                ctx.terminated = true;
                tracing::info!("shutting down...");
                ctx.resp(id.ok_resp(Empty {}))
            })
            // finally handle default req
            .unify(|req| {
                let (req, ctx) = req.split();
                tracing::warn!("unhandled {:#?}", req);
                ctx.resp(req.id.ok_resp(serde_json::Value::Null))
            })
    }

    pub fn on_notify(&mut self, notice: NotificationMessage) -> IOResult<()> {
        notice
            .with(self)
            .then(|_, _: CancelParams| {
                tracing::info!("client cancel request");
                Ok(())
            })
            .unify(|n| {
                let (notice, _) = n.split();
                tracing::warn!("unhandled {:?}", notice);
                Ok(())
            })
    }

    pub fn resp(&mut self, resp: ResponseMessage) -> IOResult<()> {
        self.codec.send(OneOf::This(resp))
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
