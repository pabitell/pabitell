use crate::{ItemState, World};
use anyhow::{anyhow, Result};

pub fn assign_item(world: &mut dyn World, item: String, state: ItemState) -> Result<()> {
    // Test whether the character is present
    let state = match state {
        ItemState::Owned(character) => {
            world
                .characters()
                .get(&character)
                .ok_or_else(|| anyhow!("Character {} not found", character))?;
            ItemState::Owned(character)
        }
        ItemState::InScene(scene) => {
            world
                .scenes()
                .get(&scene)
                .ok_or_else(|| anyhow!("Scene {} not found", scene))?;
            ItemState::InScene(scene)
        }
        i => i,
    };

    // Set item to character
    world
        .items_mut()
        .get_mut(&item)
        .ok_or_else(|| anyhow!("Item '{}' is not found", item))?
        .set_state(state);

    Ok(())
}

pub fn move_character(
    world: &mut dyn World,
    character: String,
    scene: Option<String>,
) -> Result<()> {
    // Test whether scene is present in world
    if let Some(scene) = scene.clone() {
        world
            .scenes()
            .get(&scene)
            .ok_or_else(|| anyhow!("Scene {} not found", scene))?;
    }

    // move character to scene
    world
        .characters_mut()
        .get_mut(&character)
        .ok_or_else(|| anyhow!("Character '{}' not found", character))?
        .set_scene(scene);
    Ok(())
}
