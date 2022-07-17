use actix::*;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{self, EnvFilter};
use uuid::Uuid;

use crate::websocket::{WsConnection, WsManager};

const WORKERS: usize = 8;

#[get("/ws/{namespace}/{story}/{world}/")]
async fn ws_endpoint(
    req: HttpRequest,
    data: web::Data<Addr<WsManager>>,
    path: web::Path<(String, String, Uuid)>,
    stream: web::Payload,
) -> std::result::Result<HttpResponse, Error> {
    ws::start(
        WsConnection::new(Instant::now(), path.2, data.get_ref().clone()),
        &req,
        stream,
    )
}

async fn start(port: &str) -> anyhow::Result<()> {
    // setting logging collector
    let _collector = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("PABITELL_LOG_LEVEL").unwrap_or_else(|_| "info".into()),
        )
        .pretty()
        .try_init();
    info!("Logging");

    // Start chat server actor
    let ws_manager = WsManager::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_manager.to_owned()))
            .wrap(TracingLogger::default())
            .service(ws_endpoint)
    })
    .workers(WORKERS)
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
    .map_err(|e| e.into())
}

pub fn start_web_app(port: &str) -> anyhow::Result<()> {
    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name("main-tokio")
            .build()
            .unwrap()
    })
    .block_on(start(port))
}
