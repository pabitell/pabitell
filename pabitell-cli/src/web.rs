use actix::*;
use actix_web::{
    error, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use anyhow::{anyhow, Result};
use futures::StreamExt;
use pabitell_lib::{data::EventData, Dumpable, Narrator, World};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Instant,
};
use tracing::{debug, info, Level};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{self, filter::LevelFilter, EnvFilter};
use uuid::Uuid;

use crate::{
    backend, make_story_doggie_and_kitie_cake,
    websocket::{ClientMessage as WsClientMessage, WsConnection, WsManager},
};

const WORKERS: usize = 8;
const MAX_SIZE: usize = 1024 * 1024;

#[derive(Serialize, Deserialize)]
struct NewWorld {
    id: Uuid,
}

fn make_world(_namespace: &str, story: &str) -> Option<(Box<dyn World>, Box<dyn Narrator>)> {
    match story {
        "doggie_and_kitie_cake" => make_story_doggie_and_kitie_cake(true).unwrap(),
        _ => None,
    }
}

#[post("/api/{namespace}/{story}/")]
async fn create_world(
    data: web::Data<(Db, Addr<WsManager>)>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (namespace, story) = path.into_inner();
    if let Some((world, _)) = make_world(&namespace, &story) {
        backend::store(&mut data.as_ref().0.clone(), &story, world.as_ref()).unwrap();
        HttpResponse::Created().json(NewWorld {
            id: world.id().clone(),
        })
    } else {
        HttpResponse::NotFound().json(serde_json::json!({"msg": "story not found"}))
    }
}

#[get("/api/{namespace}/{story}/{world}/")]
async fn get_world(
    data: web::Data<(Db, Addr<WsManager>)>,
    path: web::Path<(String, String, Uuid)>,
) -> impl Responder {
    let (namespace, story, world_id) = path.into_inner();
    if let Some((mut world, _)) = make_world(&namespace, &story) {
        if backend::load(
            &mut data.as_ref().0.clone(),
            &story,
            &world_id,
            world.as_mut(),
        )
        .is_ok()
        {
            HttpResponse::Ok().json(world.dump())
        } else {
            HttpResponse::NotFound().json(serde_json::json!({
                "msg": format!("world not found (id={})", world_id)
            }))
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({"msg": "story not found"}))
    }
}

#[post("/api/{namespace}/{story}/{world}/event/")]
async fn event_world(
    data: web::Data<(Db, Addr<WsManager>)>,
    path: web::Path<(String, String, Uuid)>,
    mut payload: web::Payload,
) -> std::result::Result<HttpResponse, Error> {
    let (namespace, story, world_id) = path.into_inner();

    let (mut world, narrator) = make_world(&namespace, &story).ok_or(error::ErrorNotFound(
        serde_json::json!({"msg": "story not found"}),
    ))?;

    if backend::load(
        &mut data.as_ref().0.clone(),
        &story,
        &world_id,
        world.as_mut(),
    )
    .is_err()
    {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "msg": format!("world not found (id={})", world_id)
        })));
    }

    // read payload
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // Test whether the data are JSON
    let value =
        &serde_json::from_slice(&body).map_err(|_| error::ErrorBadRequest("Expected JSON"))?;

    // Test whether it can be parsed as event
    let mut event = narrator
        .parse_event(value)
        .ok_or(error::ErrorBadRequest("Wrong event data"))?;

    // Test whether event can be triggered
    if event.can_be_triggered(world.as_ref()) {
        event.trigger(world.as_mut());
        backend::store(&mut data.as_ref().0.clone(), &story, world.as_ref()).unwrap();

        let ws_manager = data.as_ref().1.clone();
        debug!("Sending;dump={}", event.dump());
        ws_manager.do_send(WsClientMessage {
            world_id,
            data: event.dump().to_string(),
        });

        // TODO think of some reasonable retval
        Ok(HttpResponse::Ok().json(serde_json::json!({})))
    } else {
        Err(error::ErrorBadRequest("Event can't be triggered"))
    }
}

#[get("/ws/{namespace}/{story}/{world}/")]
async fn ws_endpoint(
    req: HttpRequest,
    data: web::Data<(Db, Addr<WsManager>)>,
    path: web::Path<(String, String, Uuid)>,
    stream: web::Payload,
) -> std::result::Result<HttpResponse, Error> {
    ws::start(
        WsConnection::new(Instant::now(), path.2, data.get_ref().1.clone()),
        &req,
        stream,
    )
}

async fn start(db_path: &str, port: &str) -> anyhow::Result<()> {
    // setting logging collector
    let _collector = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("PABITELL_LOG_LEVEL").unwrap_or_else(|_| "info".into()),
        )
        .pretty()
        .try_init();
    info!("Logging");

    // init db connection
    let db = sled::open(db_path)?;
    // Start chat server actor
    let ws_manager = WsManager::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new((db.clone(), ws_manager.to_owned())))
            .wrap(TracingLogger::default())
            .service(create_world)
            .service(get_world)
            .service(event_world)
            .service(ws_endpoint)
    })
    .workers(WORKERS)
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
    .map_err(|e| e.into())
}

pub fn start_web_app(db_path: &str, port: &str) -> anyhow::Result<()> {
    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name("main-tokio")
            .build()
            .unwrap()
    })
    .block_on(start(db_path, port))
}
