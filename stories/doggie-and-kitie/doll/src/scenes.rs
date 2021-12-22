use anyhow::{anyhow, Result};
use pabitell_lib::{
    conditions, scene_base, AsAny, Description, Dumpable, Id, Named, Scene, Tagged, World,
};
use serde_json::{json, Value};
use std::any::Any;
use uuid::Uuid;

use crate::{characters, translations::get_message};

#[derive(Debug, Default)]
pub struct Home {
    id: Uuid,
    dialog: usize,
}

impl Id for Home {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
}

impl Named for Home {
    fn name(&self) -> &'static str {
        "home"
    }
}

impl AsAny for Home {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Dumpable for Home {
    fn dump(&self) -> Value {
        serde_json::json!(
            {"name": self.name(), "dialog": self.dialog}
        )
    }
    fn load(&mut self, data: Value) -> Result<()> {
        if let Value::Number(number) = &data["dialog"] {
            if let Some(dialog) = number.as_u64() {
                self.dialog = dialog as usize;
            } else {
                return Err(anyhow!("Wrong dialog field '{}'", number));
            }
        } else {
            return Err(anyhow!("Scene format '{}'", self.name()));
        }

        Ok(())
    }
}

impl Tagged for Home {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
    fn set_tags(&mut self, _tags: Vec<String>) {}
}

impl Scene for Home {
    fn dialog(&self) -> Option<usize> {
        Some(self.dialog)
    }

    fn next_dialog(&mut self) {
        self.dialog += 1
    }
}

impl Description for Home {
    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-long-{}", world.name(), self.name(), self.dialog),
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

#[derive(Debug, Default)]
pub struct Walk {
    id: Uuid,
    dialog: usize,
}

impl Id for Walk {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
}

impl Named for Walk {
    fn name(&self) -> &'static str {
        "walk"
    }
}

impl AsAny for Walk {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Dumpable for Walk {
    fn dump(&self) -> Value {
        serde_json::json!(
            {"name": self.name(), "dialog": self.dialog}
        )
    }
    fn load(&mut self, data: Value) -> Result<()> {
        if let Value::Number(number) = &data["dialog"] {
            if let Some(dialog) = number.as_u64() {
                self.dialog = dialog as usize;
            } else {
                return Err(anyhow!("Wrong dialog field '{}'", number));
            }
        } else {
            return Err(anyhow!("Scene format '{}'", self.name()));
        }

        Ok(())
    }
}

impl Tagged for Walk {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
    fn set_tags(&mut self, _tags: Vec<String>) {}
}

impl Scene for Walk {
    fn dialog(&self) -> Option<usize> {
        Some(self.dialog)
    }

    fn next_dialog(&mut self) {
        self.dialog += 1
    }
}

impl Description for Walk {
    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-long-{}", world.name(), self.name(), self.dialog),
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
