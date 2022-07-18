use anyhow::{anyhow, Result};
use pabitell_lib::{AsAny, Character, Description, Dumpable, Named, Tagged};
use serde_json::{json, Value};
use std::any::Any;

#[derive(Debug, Default)]
pub struct Kitie {
    scene: Option<String>,
}

impl Tagged for Kitie {
    fn get_tags(&self) -> Vec<String> {
        vec!["cat".to_string()]
    }
}

impl Named for Kitie {
    fn name(&self) -> &'static str {
        "kitie"
    }
}

impl Description for Kitie {}

impl AsAny for Kitie {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Dumpable for Kitie {
    fn dump(&self) -> Value {
        json!(
            {
                "name": self.name(),
                "scene": self.scene,
            }
        )
    }

    fn load(&mut self, data: Value) -> Result<()> {
        match &data["scene"] {
            Value::Null => self.scene = None,
            Value::String(scene) => self.scene = Some(scene.to_string()),
            _ => return Err(anyhow!("Wrong format of character '{}'", self.name())),
        }

        Ok(())
    }
}

impl Character for Kitie {
    fn scene(&self) -> &Option<String> {
        &self.scene
    }

    fn set_scene(&mut self, scene: Option<String>) {
        self.scene = scene
    }
}

impl Kitie {}

#[derive(Debug, Default)]
pub struct Doggie {
    scene: Option<String>,
}

impl Tagged for Doggie {
    fn get_tags(&self) -> Vec<String> {
        vec!["cat".to_string()]
    }
}

impl Named for Doggie {
    fn name(&self) -> &'static str {
        "doggie"
    }
}

impl Description for Doggie {}

impl AsAny for Doggie {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Dumpable for Doggie {
    fn dump(&self) -> Value {
        json!(
            {
                "name": self.name(),
                "scene": self.scene,
            }
        )
    }

    fn load(&mut self, data: Value) -> anyhow::Result<()> {
        match &data["scene"] {
            Value::Null => self.scene = None,
            Value::String(scene) => self.scene = Some(scene.to_string()),
            _ => return Err(anyhow!("Wrong format of character '{}'", self.name())),
        }

        Ok(())
    }
}

impl Character for Doggie {
    fn scene(&self) -> &Option<String> {
        &self.scene
    }

    fn set_scene(&mut self, scene: Option<String>) {
        self.scene = scene
    }
}

impl Doggie {}

#[cfg(test)]
mod tests {
    use pabitell_lib::{World, WorldBuilder};

    use crate::world::DollWorldBuilder;

    #[test]
    fn kitie() {
        let world = DollWorldBuilder::make_world().unwrap();
        let kitie = world.characters().get("kitie").unwrap();
        assert_eq!(kitie.short(&world), "Kočička");
        assert_eq!(kitie.long(&world), "Kočička");
    }
    #[test]
    fn doggie() {
        let world = DollWorldBuilder::make_world().unwrap();
        let doggie = world.characters().get("doggie").unwrap();
        assert_eq!(doggie.short(&world), "Pejsek");
        assert_eq!(doggie.long(&world), "Pejsek");
    }
}
