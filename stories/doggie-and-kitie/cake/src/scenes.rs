use pabitell_lib::{
    AsAny, Character, Description, Event, Id, Item, Named, Scene, World, WorldBuilder,
};
use std::any::Any;
use uuid::Uuid;

use crate::translations::get_message;

#[derive(Debug, Default)]
pub struct PlayGround {
    id: Uuid,
}

impl Id for PlayGround {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for PlayGround {
    fn name(&self) -> &str {
        "playground"
    }
}

impl Description for PlayGround {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for PlayGround {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for PlayGround {}

#[derive(Debug, Default)]
pub struct Kitchen {
    id: Uuid,
}

impl Id for Kitchen {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for Kitchen {
    fn name(&self) -> &str {
        "kitchen"
    }
}

impl Description for Kitchen {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for Kitchen {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for Kitchen {}

#[derive(Debug, Default)]
pub struct Garden {
    id: Uuid,
}

impl Id for Garden {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for Garden {
    fn name(&self) -> &str {
        "garden"
    }
}

impl Description for Garden {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for Garden {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for Garden {}

#[derive(Debug, Default)]
pub struct ChildrenHouse {
    id: Uuid,
}

impl Id for ChildrenHouse {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for ChildrenHouse {
    fn name(&self) -> &str {
        "children_house"
    }
}

impl Description for ChildrenHouse {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for ChildrenHouse {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for ChildrenHouse {}

#[derive(Debug, Default)]
pub struct ChildrenGarden {
    id: Uuid,
}

impl Id for ChildrenGarden {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for ChildrenGarden {
    fn name(&self) -> &str {
        "children_garden"
    }
}

impl Description for ChildrenGarden {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for ChildrenGarden {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for ChildrenGarden {}

#[derive(Debug, Default)]
pub struct WayHome {
    id: Uuid,
}

impl Id for WayHome {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for WayHome {
    fn name(&self) -> &str {
        "way_home"
    }
}

impl Description for WayHome {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.long", world.name(), self.name()),
            world.lang(),
            None,
        )
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(
            &format!("{}.{}.short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

impl AsAny for WayHome {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for WayHome {}
