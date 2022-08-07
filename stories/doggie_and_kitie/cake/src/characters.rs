use anyhow::{anyhow, Result};
use pabitell_lib::{AsAny, Character, Description, Dumpable, Named, Tagged};
use serde_json::{json, Value};
use std::{any::Any, collections::HashSet};

#[derive(Debug, Default)]
pub struct Kitie {
    scene: Option<String>,
    pub sand_cake_last: bool, // last character to eat the sand cake
    pub consumed: HashSet<String>,
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
        let mut consumed: Vec<String> = self.consumed.iter().map(|e| e.to_owned()).collect();
        consumed.sort();
        json!(
            {
                "name": self.name(),
                "scene": self.scene,
                "sand_cake_last": self.sand_cake_last, // last character to eat the sand cake
                "consumed": consumed,
            }
        )
    }

    fn load(&mut self, data: Value) -> Result<()> {
        match &data["scene"] {
            Value::Null => self.scene = None,
            Value::String(scene) => self.scene = Some(scene.to_string()),
            _ => return Err(anyhow!("Wrong format of character '{}'", self.name())),
        }

        if let Value::Bool(value) = data["sand_cake_last"] {
            self.sand_cake_last = value;
        } else {
            return Err(anyhow!("Wrong format of character '{}'", self.name()));
        }

        if let Value::Array(consumed) = &data["consumed"] {
            self.consumed = consumed
                .into_iter()
                .map(|e| {
                    if let Value::String(meal) = e {
                        Ok(meal.to_string())
                    } else {
                        return Err(anyhow!("Wrong format of character '{}'", self.name()));
                    }
                })
                .collect::<Result<HashSet<String>>>()?;
        } else {
            return Err(anyhow!("Wrong format of character '{}'", self.name()));
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

impl Kitie {
    pub fn full(&self) -> bool {
        self.consumed
            == vec!["meat", "dumplings", "soup", "pie"]
                .into_iter()
                .map(|e| e.to_owned())
                .collect()
    }
}

#[derive(Debug, Default)]
pub struct Doggie {
    scene: Option<String>,
    pub sand_cake_last: bool, // last character to eat the sand cake
    pub consumed: HashSet<String>,
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
        let mut consumed: Vec<String> = self.consumed.iter().map(|e| e.to_owned()).collect();
        consumed.sort();
        json!(
            {
                "name": self.name(),
                "scene": self.scene,
                "sand_cake_last": self.sand_cake_last, // last character to eat the sand cake
                "consumed": consumed,
            }
        )
    }

    fn load(&mut self, data: Value) -> anyhow::Result<()> {
        match &data["scene"] {
            Value::Null => self.scene = None,
            Value::String(scene) => self.scene = Some(scene.to_string()),
            _ => return Err(anyhow!("Wrong format of character '{}'", self.name())),
        }

        if let Value::Bool(value) = data["sand_cake_last"] {
            self.sand_cake_last = value;
        } else {
            return Err(anyhow!("Wrong format of character '{}'", self.name()));
        }

        if let Value::Array(consumed) = &data["consumed"] {
            self.consumed = consumed
                .into_iter()
                .map(|e| {
                    if let Value::String(meal) = e {
                        Ok(meal.to_string())
                    } else {
                        return Err(anyhow!("Wrong format of character '{}'", self.name()));
                    }
                })
                .collect::<Result<HashSet<String>>>()?;
        } else {
            return Err(anyhow!("Wrong format of character '{}'", self.name()));
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

impl Doggie {
    pub fn full(&self) -> bool {
        self.consumed
            == vec!["meat", "dumplings", "soup", "pie"]
                .into_iter()
                .map(|e| e.to_owned())
                .collect()
    }
}

#[cfg(test)]
mod tests {
    use pabitell_lib::{World, WorldBuilder};

    use crate::world::CakeWorldBuilder;

    #[test]
    fn kitie() {
        let world = CakeWorldBuilder::make_world().unwrap();
        let kitie = world.characters().get("kitie").unwrap();
        assert_eq!(kitie.short(&world), "Kočička");
        assert_eq!(kitie.long(&world), "Kočička");
    }
    #[test]
    fn doggie() {
        let world = CakeWorldBuilder::make_world().unwrap();
        let doggie = world.characters().get("doggie").unwrap();
        assert_eq!(doggie.short(&world), "Pejsek");
        assert_eq!(doggie.long(&world), "Pejsek");
    }
}
