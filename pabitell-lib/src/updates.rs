use crate::{ItemState, World};
use anyhow::{anyhow, Result};
use std::fmt;

pub trait Change: fmt::Debug {
    fn change(&self, world: &mut dyn World) -> Result<()>;
}

pub struct AssignItemChange {
    item: String,
    state: ItemState,
}

impl AssignItemChange {
    pub fn new(item: String, state: ItemState) -> Self {
        Self { item, state }
    }
}

impl fmt::Debug for AssignItemChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssignItem")
            .field("item", &self.item)
            .field("state", &self.state)
            .finish()
    }
}

impl Change for AssignItemChange {
    fn change(&self, world: &mut dyn World) -> Result<()> {
        let state = match &self.state {
            ItemState::Owned(character) => {
                world
                    .characters()
                    .get(character)
                    .ok_or_else(|| anyhow!("Character {} not found", character))?;
                ItemState::Owned(character.to_string())
            }
            ItemState::InScene(scene) => {
                world
                    .scenes()
                    .get(scene)
                    .ok_or_else(|| anyhow!("Scene {} not found", scene))?;
                ItemState::InScene(scene.to_string())
            }
            i => i.clone(),
        };

        // Set item to character
        let item = world
            .items_mut()
            .get_mut(&self.item)
            .ok_or_else(|| anyhow!("Item '{}' is not found", self.item))?;

        item.set_state(state);

        Ok(())
    }
}

pub struct MoveCharacterChange {
    character: String,
    scene: Option<String>,
}

impl MoveCharacterChange {
    pub fn new(character: String, scene: Option<String>) -> Self {
        Self { character, scene }
    }
}

impl fmt::Debug for MoveCharacterChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MoveCharacter")
            .field("character", &self.character)
            .field("scene", &self.scene)
            .finish()
    }
}

impl Change for MoveCharacterChange {
    fn change(&self, world: &mut dyn World) -> Result<()> {
        // Test whether scene is present in world
        if let Some(scene) = self.scene.clone() {
            world
                .scenes()
                .get(&scene)
                .ok_or_else(|| anyhow!("Scene {} not found", scene))?;
        }

        // move character to scene
        world
            .characters_mut()
            .get_mut(&self.character)
            .ok_or_else(|| anyhow!("Character '{}' not found", &self.character))?
            .set_scene(self.scene.clone());
        Ok(())
    }
}

pub struct NextSceneDialogChange {
    scene: String,
}

impl NextSceneDialogChange {
    pub fn new(scene: String) -> Self {
        Self { scene }
    }
}

impl fmt::Debug for NextSceneDialogChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NextSceneDialog")
            .field("scene", &self.scene)
            .finish()
    }
}

impl Change for NextSceneDialogChange {
    fn change(&self, world: &mut dyn World) -> Result<()> {
        world
            .scenes_mut()
            .get_mut(&self.scene)
            .ok_or_else(|| anyhow!("Scene {} not found", self.scene))?
            .next_dialog();
        Ok(())
    }
}
