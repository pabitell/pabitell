use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use anyhow::{anyhow, Result};
use futures::StreamExt;
use pabitell_lib::{data::EventData, Dumpable};
use serde::{Deserialize, Serialize};
use sled::Db;
use uuid::Uuid;

use crate::{backend, make_story_doggie_and_kitie_cake};

const WORKERS: usize = 8;
const MAX_SIZE: usize = 1024 * 1024;

#[derive(Serialize, Deserialize)]
struct NewWorld {
    id: Uuid,
}

#[post("/{namespace}/{story}/")]
async fn create_world(data: web::Data<Db>, path: web::Path<(String, String)>) -> impl Responder {
    let (_namespace, story) = path.into_inner();
    let (world, _) = match story.as_str() {
        "doggie_and_kitie_cake" => make_story_doggie_and_kitie_cake(true).unwrap().unwrap(),
        _ => unreachable!(),
    };
    backend::store(&mut data.as_ref().clone(), &story, world.as_ref()).unwrap();
    HttpResponse::Created().json(NewWorld {
        id: world.id().clone(),
    })
}

#[get("/{namespace}/{story}/{world}/")]
async fn get_world(data: web::Data<Db>, path: web::Path<(String, String, Uuid)>) -> impl Responder {
    let (_namespace, story, world_id) = path.into_inner();
    let (mut world, _) = match story.as_str() {
        "doggie_and_kitie_cake" => make_story_doggie_and_kitie_cake(false).unwrap().unwrap(),
        _ => unreachable!(),
    };
    backend::load(
        &mut data.as_ref().clone(),
        &story,
        &world_id,
        world.as_mut(),
    )
    .unwrap();
    HttpResponse::Ok().json(world.dump())
}

#[post("/{namespace}/{story}/{world}/event/")]
async fn event_world(
    data: web::Data<Db>,
    path: web::Path<(String, String, Uuid)>,
    mut payload: web::Payload,
) -> std::result::Result<HttpResponse, Error> {
    let (_namespace, story, world_id) = path.into_inner();
    let (mut world, narrator) = match story.as_str() {
        "doggie_and_kitie_cake" => make_story_doggie_and_kitie_cake(false).unwrap().unwrap(),
        _ => unreachable!(),
    };
    backend::load(
        &mut data.as_ref().clone(),
        &story,
        &world_id,
        world.as_mut(),
    )
    .unwrap();

    // read payload
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // Test whether it matches available_events
    if let Ok(value) = serde_json::from_slice(&body) {
        if let Some(mut event) = narrator.parse_event(value) {
            if event.can_be_triggered(world.as_ref()) {
                event.trigger(world.as_mut());
                backend::store(&mut data.as_ref().clone(), &story, world.as_ref()).unwrap();
                Ok(HttpResponse::Ok().finish())
            } else {
                Err(error::ErrorBadRequest("Event can't be triggered"))
            }
        } else {
            Err(error::ErrorBadRequest("Wrong event data"))
        }
    } else {
        Err(error::ErrorBadRequest("Expected JSON"))
    }
}

async fn start(db_path: &str, port: &str) -> anyhow::Result<()> {
    let db = sled::open(db_path)?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(create_world)
            .service(get_world)
            .service(event_world)
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
