use crate::World;
use anyhow::{anyhow, Result};
use sled::Db;
use uuid::Uuid;

pub fn list_stored(db: &Db, story: &str) -> Result<Vec<Uuid>> {
    let tree = db.open_tree(story)?;
    let results: Vec<Uuid> = tree
        .scan_prefix(&[])
        .keys()
        .filter_map(|e| {
            if let Ok(e) = e {
                if let Ok(id) = Uuid::from_slice(&e[..]) {
                    Some(id)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    Ok(results)
}

pub fn load(db: &Db, story: &str, uuid: &Uuid, world: &mut dyn World) -> Result<()> {
    let tree = db.open_tree(story)?;
    world.set_id(*uuid);
    let data = tree
        .get(uuid.as_bytes())?
        .ok_or_else(|| anyhow!("No data"))?;
    let json = serde_json::from_slice(&data[..])?;
    world.load(json)?;
    Ok(())
}

pub fn delete(db: &mut Db, story: &str, uuid: &Uuid) -> Result<()> {
    let tree = db.open_tree(story)?;
    tree.remove(uuid.as_bytes())?;
    tree.flush()?;
    Ok(())
}

pub fn store(db: &mut Db, story: &str, world: &dyn World) -> Result<()> {
    let tree = db.open_tree(story)?;
    tree.insert(world.id().as_bytes(), serde_json::to_vec(&world.dump())?)?;
    tree.flush()?;
    Ok(())
}
