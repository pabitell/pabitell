use rexie::{Index, KeyRange, ObjectStore, Result, Rexie, TransactionMode};
use serde_json::Value;
use uuid::Uuid;
use wasm_bindgen::JsValue;

pub async fn init_database(story_name: &str) -> Result<Rexie> {
    Rexie::builder(story_name)
        .version(1)
        .add_object_store(
            ObjectStore::new("worlds")
                .key_path("id")
                .auto_increment(false),
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

pub async fn get_world(rex: &Rexie, id: &Uuid) -> Result<Option<Value>> {
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadOnly)?;

    let worlds = transaction.store("worlds")?;
    let world = worlds.get(&JsValue::from_str(&id.to_string())).await?;
    if world.is_null() {
        return Ok(None);
    }
    let world_json = world.into_serde().unwrap();
    transaction.commit().await?;
    Ok(Some(world_json))
}

pub async fn get_worlds(rex: &Rexie) -> Result<Vec<(Uuid, Value)>> {
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadOnly)?;

    let worlds = transaction.store("worlds")?;

    let worlds = worlds.get_all(None, None, None, None).await?;

    transaction.commit().await?;
    Ok(worlds
        .iter()
        .map(|(k, v)| {
            (
                k.as_string().unwrap().parse().unwrap(),
                v.into_serde().unwrap(),
            )
        })
        .collect())
}

pub async fn set_world(rex: &Rexie, id: &Uuid, data: Value) -> Result<()> {
    let transaction = rex.transaction(&["worlds"], TransactionMode::ReadWrite)?;
    let worlds = transaction.store("worlds")?;
    let world_id = id.to_string();
    let record = serde_json::json!({
        "id":  world_id,
        "data": data,
    });
    worlds
        .put(&JsValue::from_serde(&Some(record)).unwrap(), None)
        .await?;
    transaction.commit().await?;

    Ok(())
}

pub async fn get_events(rex: &Rexie, world_id: &Uuid) -> Result<Vec<Value>> {
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

    transaction.commit().await?;

    Ok(world_events
        .iter()
        .map(|(_, v)| v.into_serde().unwrap())
        .collect())
}

pub async fn get_event(rex: &Rexie, world_id: &Uuid, idx: u64) -> Result<Option<Value>> {
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
    transaction.commit().await?;

    Ok(event_json)
}

pub async fn set_event(rex: &Rexie, world_id: &Uuid, idx: u64, data: Value) -> Result<()> {
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
