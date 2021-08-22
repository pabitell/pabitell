use crate::{ItemState, World};
use anyhow::Result;
use std::collections::HashSet;

pub fn same_scene(world: &dyn World, characters: &[String], items: &[String]) -> Result<bool> {
    let character_scenes = characters
        .iter()
        .filter_map(|e| world.characters().get(e.as_str())?.scene().clone())
        .collect::<Vec<_>>();
    let item_scenes = items
        .iter()
        .filter_map(|e| match world.items().get(e.as_str())?.state() {
            ItemState::InScene(scene) => Some(scene),
            _ => None,
        })
        .collect::<Vec<_>>();
    // All characters and items are in some scene
    if character_scenes.len() == characters.len() && item_scenes.len() == items.len() {
        if character_scenes
            .iter()
            .chain(item_scenes.into_iter())
            .collect::<HashSet<_>>()
            .len()
            < 2
        {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn has_item(world: &dyn World, character: String, item: String) -> Result<bool> {
    if let Some(item) = world.items().get(&item) {
        match item.state() {
            ItemState::Owned(owner) => Ok(&character == owner),
            _ => Ok(false),
        }
    } else {
        Ok(false)
    }
}

pub fn in_scenes(world: &dyn World, character: String, scenes: &[String]) -> Result<bool> {
    Ok(
        if let Some(character) = world.characters().get(&character) {
            if let Some(character_scene) = character.scene() {
                scenes.contains(character_scene)
            } else {
                false
            }
        } else {
            false
        },
    )
}

pub fn can_give(
    world: &dyn World,
    from_character: String,
    to_character: String,
    item: String,
) -> Result<bool> {
    Ok(has_item(world, from_character.clone(), item)?
        && same_scene(world, &vec![from_character, to_character], &vec![])?)
}
