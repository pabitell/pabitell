use pabitell_lib::{
    AsAny, Character, Description, Event, Id, Item, Named, Scene, World, WorldBuilder,
};
use std::any::Any;
use uuid::Uuid;

use crate::translations::get_message;

#[derive(Debug, Default)]
pub struct Kitie {
    id: Uuid,
    scene: Option<&'static str>,
    pub sand_cake_last: bool, // last character to eat the sand cake
    pub consumed_pie: bool,
    pub consumed_soup: bool,
    pub consumed_dumplings: bool,
    pub consumed_meat: bool,
}

impl Id for Kitie {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
    fn roles(&self) -> Vec<&'static str> {
        vec!["cat"]
    }
}

impl Named for Kitie {
    fn name(&self) -> &'static str {
        "kitie"
    }
}

impl Description for Kitie {
    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for Kitie {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Character for Kitie {
    fn scene(&self) -> Option<&'static str> {
        self.scene
    }

    fn set_scene(&mut self, scene: Option<&'static str>) {
        self.scene = scene
    }
}

impl Kitie {
    pub fn full(&self) -> bool {
        self.consumed_meat && self.consumed_dumplings && self.consumed_soup && self.consumed_pie
    }
}

#[derive(Debug, Default)]
pub struct Doggie {
    id: Uuid,
    scene: Option<&'static str>,
    pub sand_cake_last: bool, // last character to eat the sand cake
    pub consumed_pie: bool,
    pub consumed_soup: bool,
    pub consumed_dumplings: bool,
    pub consumed_meat: bool,
}

impl Id for Doggie {
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
    fn roles(&self) -> Vec<&'static str> {
        vec!["dog"]
    }
}

impl Named for Doggie {
    fn name(&self) -> &'static str {
        "doggie"
    }
}

impl Description for Doggie {
    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for Doggie {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Character for Doggie {
    fn scene(&self) -> Option<&'static str> {
        self.scene
    }

    fn set_scene(&mut self, scene: Option<&'static str>) {
        self.scene = scene
    }
}

impl Doggie {
    pub fn full(&self) -> bool {
        self.consumed_meat && self.consumed_dumplings && self.consumed_soup && self.consumed_pie
    }
}

#[cfg(test)]
mod tests {
    use pabitell_lib::{World, WorldBuilder};

    use crate::CakeWorldBuilder;

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
