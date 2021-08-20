use pabitell_lib::{
    AsAny, Character, Description, Event, Id, Item, ItemState, Named, Scene, World, WorldBuilder,
};
use std::any::Any;
use uuid::Uuid;

use crate::{characters, translations::get_message};

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
    fn name(&self) -> &'static str {
        "playground"
    }
}

impl Description for PlayGround {
    fn long(&self, world: &dyn World) -> String {
        match world.items().get("sand_cake").unwrap().state() {
            ItemState::Owned(character) => match character.as_str() {
                "doggie" => get_message(
                    &format!("{}-{}-doggie_pick", world.name(), self.name()),
                    world.lang(),
                    None,
                ),
                "kitie" => get_message(
                    &format!("{}-{}-kitie_pick", world.name(), self.name()),
                    world.lang(),
                    None,
                ),
                _ => unreachable!(),
            },
            ItemState::InScene(_) => get_message(
                &format!("{}-{}-initial", world.name(), self.name()),
                world.lang(),
                None,
            ),
            ItemState::Unassigned => {
                let doggie = world
                    .characters()
                    .get("doggie")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<characters::Doggie>()
                    .unwrap();

                let kitie = world
                    .characters()
                    .get("kitie")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<characters::Kitie>()
                    .unwrap();

                if doggie.sand_cake_last {
                    return get_message(
                        &format!("{}-{}-doggie_last", world.name(), self.name()),
                        world.lang(),
                        None,
                    );
                }

                if kitie.sand_cake_last {
                    return get_message(
                        &format!("{}-{}-kitie_last", world.name(), self.name()),
                        world.lang(),
                        None,
                    );
                }

                unreachable!()
            }
        }
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
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
    fn name(&self) -> &'static str {
        "kitchen"
    }
}

impl Description for Kitchen {
    fn long(&self, world: &dyn World) -> String {
        if world
            .items()
            .values()
            .filter(|e| e.roles().contains(&"accepted"))
            .all(|e| e.state() == &ItemState::Unassigned)
        {
            get_message(
                &format!("{}-{}-ready", world.name(), self.name()),
                world.lang(),
                None,
            )
        } else {
            get_message(
                &format!("{}-{}-ingredients", world.name(), self.name()),
                world.lang(),
                None,
            )
        }
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
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
    fn name(&self) -> &'static str {
        "garden"
    }
}

impl Description for Garden {
    fn long(&self, world: &dyn World) -> String {
        if world.items().get("bad_dog").unwrap().state() == &ItemState::Unassigned {
            get_message(
                &format!("{}-{}-found", world.name(), self.name()),
                world.lang(),
                None,
            )
        } else {
            get_message(
                &format!("{}-{}-searching", world.name(), self.name()),
                world.lang(),
                None,
            )
        }
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
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
    fn name(&self) -> &'static str {
        "children_house"
    }
}

impl Description for ChildrenHouse {
    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-eating", world.name(), self.name()),
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
    fn name(&self) -> &'static str {
        "children_garden"
    }
}

impl Description for ChildrenGarden {
    fn long(&self, world: &dyn World) -> String {
        if world
            .items()
            .values()
            .filter(|e| e.roles().contains(&"toy"))
            .all(|e| e.state() == &ItemState::Unassigned)
        {
            get_message(
                &format!("{}-{}-leaving", world.name(), self.name()),
                world.lang(),
                None,
            )
        } else {
            get_message(
                &format!("{}-{}-playing", world.name(), self.name()),
                world.lang(),
                None,
            )
        }
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
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
    fn name(&self) -> &'static str {
        "way_home"
    }
}

impl Description for WayHome {
    fn long(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-end", world.name(), self.name()),
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

impl AsAny for WayHome {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Scene for WayHome {}
