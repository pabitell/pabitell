use chrono::{DateTime, Utc};
use rexie::{Index, KeyRange, ObjectStore, Result, Rexie, TransactionMode};
use serde_json::Value;
use uuid::Uuid;
use wasm_bindgen::JsValue;

async fn _init_database(name: &str) -> Result<Rexie> {
    Rexie::builder(name)
        .version(1)
        .add_object_store(
            ObjectStore::new("worlds")
                .key_path("id")
                .auto_increment(false)
                .add_index(Index::new("last", "last")),
        )
        .add_object_store(
            ObjectStore::new("events")
                .key_path("id")
                .auto_increment(true)
                .add_index(Index::new("world_id", "world_id"))
                .add_index(Index::new("idx", "idx")),
        )
        .build()
        .await
}

pub async fn init_database(name: &str) -> Rexie {
    if let Ok(db) = _init_database(name).await {
        log::debug!("DB obtained '{name}'");
        db
    } else {
        log::warn!("DB wiping '{name}'");
        Rexie::delete(name).await.unwrap();
        log::info!("Recreating database '{name}'");
        _init_database(name).await.unwrap()
    }
}

pub async fn get_world(rex: &Rexie, id: &Uuid) -> Result<Option<Value>> {
    log::debug!("DB Get world '{id}'");
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadOnly)?;

    let worlds = transaction.store("worlds")?;
    let world = worlds.get(&JsValue::from_str(&id.to_string())).await?;
    if world.is_null() {
        return Ok(None);
    }
    let world_json = world.into_serde().unwrap();
    transaction.done().await?;
    Ok(Some(world_json))
}

pub async fn get_worlds(rex: &Rexie) -> Result<Vec<(DateTime<Utc>, Uuid, String, bool, Value)>> {
    log::debug!("DB Get worlds '{}'", rex.name());
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadOnly)?;

    let worlds = transaction.store("worlds")?;
    let index = worlds.index("last")?;

    let worlds = index.get_all(None, None, None, None).await?;

    transaction.done().await?;
    Ok(worlds
        .iter()
        .map(|(k, v)| {
            let data: Value = v.into_serde().unwrap();
            let character = data["character"]
                .as_str()
                .map(|e| e.to_string())
                .unwrap_or_else(|| "narrator".to_string());
            let fixed_character = data["fixed_character"].as_bool().unwrap_or(false);
            (
                k.as_string().unwrap().parse::<DateTime<Utc>>().unwrap(),
                data["id"].as_str().unwrap().parse::<Uuid>().unwrap(),
                character,
                fixed_character,
                data["data"].clone(),
            )
        })
        .collect())
}

pub async fn put_world(
    rex: &Rexie,
    id: &Uuid,
    character: String,
    fixed_character: bool,
    data: Value,
) -> Result<()> {
    log::debug!("DB Put world '{id}'");
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadWrite)?;
    let worlds = transaction.store("worlds")?;
    let world_id = id.to_string();
    let record = serde_json::json!({
        "id":  world_id,
        "data": data,
        "character": character,
        "fixed_character": fixed_character,
        "last": Utc::now(),
    });
    worlds
        .put(&JsValue::from_serde(&Some(record)).unwrap(), None)
        .await?;
    transaction.done().await?;

    Ok(())
}

pub async fn del_world(rex: &Rexie, id: &Uuid) -> Result<()> {
    log::debug!("DB delete world '{id}'");
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadWrite)?;
    let worlds = transaction.store("worlds")?;
    worlds.delete(&JsValue::from_str(&id.to_string())).await?;

    transaction.done().await?;
    Ok(())
}

pub async fn get_events(rex: &Rexie, world_id: &Uuid) -> Result<Vec<Value>> {
    log::debug!("DB Get events");
    let transaction = rex.transaction(&["events"], TransactionMode::ReadOnly)?;

    let events = transaction.store("events")?;
    let world_events = events
        .index("world_id")?
        .get_all(
            Some(&KeyRange::only(&JsValue::from_str(&world_id.to_string()))?),
            None,
            None,
            None,
        )
        .await?;

    transaction.done().await?;

    Ok(world_events
        .iter()
        .map(|(_, v)| v.into_serde().unwrap())
        .collect())
}

pub async fn get_event(rex: &Rexie, world_id: &Uuid, idx: u64) -> Result<Option<Value>> {
    log::debug!("DB Get event '{world_id}' - '{idx}'");
    let transaction = rex.transaction(&["events"], TransactionMode::ReadOnly)?;

    let events = transaction.store("events")?;
    let world_events = events
        .index("world_id")?
        .get_all(
            Some(&KeyRange::only(&JsValue::from_str(&world_id.to_string()))?),
            None,
            None,
            None,
        )
        .await?;

    let mut res: Vec<_> = world_events
        .iter()
        .filter_map(|(_, v)| v.into_serde().ok())
        .filter_map(|v: Value| {
            if let Value::Number(num) = &v["idx"] {
                if num.as_u64() == Some(idx) {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let event_json = if res.is_empty() {
        None
    } else if res.len() > 1 {
        // Disambigous
        unreachable!()
    } else {
        Some(res.remove(0))
    };
    transaction.done().await?;

    Ok(event_json)
}

pub async fn put_event(rex: &Rexie, world_id: &Uuid, idx: u64, data: Value) -> Result<()> {
    log::debug!("DB put event '{world_id}' - '{idx}'");

    let existing = get_event(rex, world_id, idx).await?;
    let transaction = rex.transaction(&["events"], TransactionMode::ReadWrite)?;
    let events = transaction.store("events")?;
    let record = serde_json::json!({
        "idx": idx,
        "world_id": world_id.to_string(),
        "data": data,
    });

    if let Some(existing_event) = existing {
        let id = &existing_event["id"];
        if let Value::Number(num) = id {
            if let Some(idx) = num.as_f64() {
                events
                    .put(
                        &JsValue::from_serde(&record).unwrap(),
                        Some(&JsValue::from_f64(idx)),
                    )
                    .await?;
            } else {
                unreachable!();
            }
        } else {
            // This should not happen
            unreachable!();
        }
    } else {
        events
            .put(&JsValue::from_serde(&record).unwrap(), None)
            .await?;
    }

    Ok(())
}
