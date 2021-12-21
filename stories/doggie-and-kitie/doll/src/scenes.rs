use anyhow::{anyhow, Result};
use pabitell_lib::{
    conditions, AsAny, Character, Description, Dumpable, Event, Id, Item, ItemState, Named, Scene,
    Tagged, World, WorldBuilder,
};
use serde_json::{json, Value};
use std::any::Any;
use uuid::Uuid;

use crate::{characters, translations::get_message};

// TODO this can go to pabitell_lib
macro_rules! scene_base {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: Uuid,
        }

        impl Id for $class_name {
            fn id(&self) -> &Uuid {
                &self.id
            }

            fn set_id(&mut self, id: Uuid) {
                self.id = id;
            }
        }

        impl Named for $class_name {
            fn name(&self) -> &'static str {
                $name
            }
        }

        impl Tagged for $class_name {
            fn get_tags(&self) -> Vec<String> {
                #[allow(unused_mut)]
                let mut res: Vec<String> = vec![];
                $(
                    res.push($tag.into());
                )*
                res
            }
        }

        impl AsAny for $class_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }


        impl Dumpable for $class_name {
            fn dump(&self) -> Value {
                json!(
                    {"name": self.name()}
                )
            }
            fn load(&mut self, data: Value) -> Result<()> {
                Ok(())  // Scenes doesn't cotain any extras here
            }
        }

        impl Scene for $class_name {}
    };
}

scene_base!(Home, "home", []);

impl Description for Home {
    fn long(&self, world: &dyn World) -> String {
        /*
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
        */
        unreachable!()
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}
