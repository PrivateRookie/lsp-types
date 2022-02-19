use std::{borrow::BorrowMut, sync::Arc};

use clap::Parser;
use lsp_io::AMessageCodec;
use lsp_ty::{
    CancelParams, CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, Empty,
    InitializeParams, InitializeResult, InitializeResultServerInfo, NotificationMessage, OneOf,
    OneOf3, ReqId, RequestMessage, ResponseMessage, ServerCapabilities, ShutdownParams,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

pub type IOResult<T> = std::io::Result<T>;

pub struct Server {
    codec: AMessageCodec<TcpStream>,
    terminated: bool,
}

impl Server {
    fn new(stream: TcpStream) -> Self {
        Self {
            codec: AMessageCodec::new(stream),
            terminated: false,
        }
    }

    async fn receive(&mut self) -> IOResult<()> {
        match self.codec.receive().await? {
            OneOf3::This(req) => self.on_req(req).await,
            OneOf3::Among(resp) => {
                tracing::info!("{:#?}", resp);
                Ok(())
            }
            OneOf3::Other(notice) => self.on_notify(notice),
        }
    }

    pub async fn on_req(&mut self, req: RequestMessage) -> IOResult<()> {
        // pass context when handle request need;

        let ctx = req.with(Arc::new(Mutex::new(self)));
        ctx
            // pass handler function, you must specify param type
            // in anonymous handler function argument, other wise
            // you have to use turbo fish symbol
            .then(|ctx, id, _: InitializeParams| {
                let blk = async move {
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
                    ctx.lock().await.borrow_mut().resp(id.ok_resp(ret)).await
                };
                Box::pin(blk)
            })
            // use or_else to route to other handler function if
            // method do not match
            .or_else(|ctx, id, _: CompletionParams| {
                let blk = async move {
                    let item = CompletionItem {
                        label: "demo".to_string(),
                        detail: Some("that's ok".to_string()),
                        insert_text: Some("yaya".to_string()),
                        kind: Some(CompletionItemKind::Keyword),
                        ..Default::default()
                    };
                    ctx.lock()
                        .await
                        .borrow_mut()
                        .resp(id.ok_resp(vec![item]))
                        .await
                };
                Box::pin(blk)
            })
            // .or_else(|ctx, id, _: ShutdownParams| {
            //     ctx.terminated = true;
            //     tracing::info!("shutting down...");
            //     ctx.lock().await.resp(id.ok_resp(Empty {})).await
            // })
            // finally handle default req
            .unify(|req| {
                let blk = async move {
                    let (req, ctx) = req.split();
                    tracing::warn!("unhandled {:#?}", req);
                    ctx.lock()
                        .await
                        .resp(req.id.ok_resp(serde_json::Value::Null))
                        .await
                };
                Box::pin(blk)
            })
            .await
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

    pub async fn resp(&mut self, resp: ResponseMessage) -> IOResult<()> {
        self.codec.send_resp(resp).await
    }

    pub async fn notify(&mut self, msg: NotificationMessage) -> IOResult<()> {
        self.codec.send_notice(msg).await
    }

    pub async fn run(&mut self) -> IOResult<()> {
        while !self.terminated {
            self.receive().await?;
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

#[tokio::main]
async fn main() -> IOResult<()> {
    let args = Args::parse();
    init_log(args.level);
    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .expect("failed to bind addr");
    tracing::info!("listening at {}:{}", args.host, args.port);
    for (conn, _) in listener.accept().await {
        let mut server = Server::new(conn);
        tracing::info!("launching new lsp server...");
        server.run().await?;
    }
    Ok(())
}
