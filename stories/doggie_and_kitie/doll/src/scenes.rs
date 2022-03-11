use anyhow::anyhow;
use pabitell_lib::{
    scene_base, scene_no_music, scene_with_dialog, AsAny, Description, ItemState, Music, Named,
    Scene, World,
};
use serde_json::Value;
use std::any::Any;

use crate::translations::get_message;

scene_with_dialog!(Home, "home", []);
scene_no_music!(Home);

scene_with_dialog!(Walk, "walk", []);

impl Music for Walk {
    fn music(&self) -> Option<String> {
        if self.dialog < 5 {
            Some("music/crying.ogg".to_owned())
        } else {
            None
        }
    }
}

scene_base!(DoggieSeach, "doggie_search", []);
scene_no_music!(DoggieSeach);

impl Description for DoggieSeach {
    fn long(&self, world: &dyn World) -> String {
        let start = world.items().values().any(|e| {
            e.get_tags().contains(&"doggie_pick".to_owned())
                && e.state() == &ItemState::InScene("doggie_search".to_owned())
        });
        get_message(
            &format!(
                "{}-{}-long-{}",
                world.name(),
                self.name(),
                if start { "start" } else { "end" }
            ),
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
scene_no_music!(KitieSeach);

impl Description for KitieSeach {
    fn long(&self, world: &dyn World) -> String {
        let start = world.items().values().any(|e| {
            e.get_tags().contains(&"kitie_pick".to_owned())
                && e.state() == &ItemState::InScene("kitie_search".to_owned())
        });
        get_message(
            &format!(
                "{}-{}-long-{}",
                world.name(),
                self.name(),
                if start { "start" } else { "end" }
            ),
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
