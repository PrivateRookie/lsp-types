use std::{borrow::BorrowMut, sync::Arc};

use clap::Parser;
use lsp_io::AsyncCodec;
use lsp_ty::{
    CancelParams, CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, Empty,
    InitializeParams, InitializeResult, InitializeResultServerInfo, NotificationMessage, OneOf3,
    RequestMessage, ResponseMessage, ServerCapabilities, ShutdownParams,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

pub type IOResult<T> = std::io::Result<T>;

pub struct Server {
    codec: AsyncCodec<TcpStream>,
    terminated: bool,
}

impl Server {
    fn new(stream: TcpStream) -> Self {
        Self {
            codec: AsyncCodec::new(stream),
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
            OneOf3::Other(notice) => self.on_notify(notice).await,
        }
    }

    pub async fn on_req(&mut self, req: RequestMessage) -> IOResult<()> {
        // pass context when handle request need;

        req.with(Arc::new(Mutex::new(self)))
            // pass handler function, you must specify param type
            // in anonymous handler function argument, other wise
            // you have to use turbo fish symbol
            .async_then(|ctx, id, _: InitializeParams| async move {
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
            })
            .await
            // use or_else to route to other handler function if
            // method do not match
            .async_or_else(|ctx, id, _: CompletionParams| async move {
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
            })
            .await
            .async_or_else(|ctx, id, _: ShutdownParams| async move {
                let mut ctx = ctx.lock().await;
                ctx.terminated = true;
                tracing::info!("shutting down...");
                ctx.resp(id.ok_resp(Empty {})).await
            })
            // finally handle default req
            .await
            .async_unify(|req| async move {
                let (req, ctx) = req.split();
                tracing::warn!("unhandled {:#?}", req);
                let mut ctx = ctx.lock().await;
                ctx.resp(req.id.ok_resp(serde_json::Value::Null)).await
            })
            .await
    }

    pub async fn on_notify(&mut self, notice: NotificationMessage) -> IOResult<()> {
        notice
            .with(Arc::new(Mutex::new(self)))
            .async_then(|_, _: CancelParams| async move {
                tracing::info!("client cancel request");
                Ok(())
            })
            .await
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
