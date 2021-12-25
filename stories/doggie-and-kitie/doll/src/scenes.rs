use anyhow::anyhow;
use pabitell_lib::{scene_base, scene_with_dialog, AsAny, Description, Named, World};
use serde_json::Value;
use std::any::Any;

use crate::translations::get_message;

scene_with_dialog!(Home, "home", []);
scene_with_dialog!(Walk, "walk", []);

scene_base!(DoggieSeach, "doggie_search", []);

impl Description for DoggieSeach {
    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

scene_base!(KitieSeach, "kitie_search", []);

impl Description for KitieSeach {
    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}
