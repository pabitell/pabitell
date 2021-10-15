use anyhow::{anyhow, Result};
use pabitell_lib::{
    conditions, AsAny, Character, Description, Dumpable, Event, Id, Item, ItemState, Named, Scene,
    Tagged, World, WorldBuilder,
};
use serde_json::{json, Value};
use std::any::Any;
use uuid::Uuid;

use crate::{characters, translations::get_message};

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

scene_base!(PlayGround, "playground", []);

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

scene_base!(Kitchen, "kitchen", []);

impl Description for Kitchen {
    fn long(&self, world: &dyn World) -> String {
        let items = world.items().values().clone().collect::<Vec<_>>();

        let batch1_ready = conditions::all_items_with_tags_in_state(
            world,
            &["batch1".to_string()],
            ItemState::Unassigned,
        );
        let batch2_ready = conditions::all_items_with_tags_in_state(
            world,
            &["batch1".to_string(), "batch2".to_string()],
            ItemState::Unassigned,
        );
        let batch3_ready = conditions::all_items_with_tags_in_state(
            world,
            &[
                "batch1".to_string(),
                "batch2".to_string(),
                "batch3".to_string(),
            ],
            ItemState::Unassigned,
        );
        let batch4_ready = conditions::all_items_with_tags_in_state(
            world,
            &[
                "batch1".to_string(),
                "batch2".to_string(),
                "batch3".to_string(),
                "batch4".to_string(),
            ],
            ItemState::Unassigned,
        );
        let batch5_ready = conditions::all_items_with_tags_in_state(
            world,
            &[
                "batch1".to_string(),
                "batch2".to_string(),
                "batch3".to_string(),
                "batch4".to_string(),
                "batch5".to_string(),
            ],
            ItemState::Unassigned,
        );
        let batch6_ready = conditions::all_items_with_tags_in_state(
            world,
            &[
                "batch1".to_string(),
                "batch2".to_string(),
                "batch3".to_string(),
                "batch4".to_string(),
                "batch5".to_string(),
            ],
            ItemState::Unassigned,
        );

        let message = if !batch1_ready {
            format!("{}-{}-ingredients-batch1", world.name(), self.name())
        } else if !batch2_ready {
            format!("{}-{}-ingredients-batch2", world.name(), self.name())
        } else if !batch3_ready {
            format!("{}-{}-ingredients-batch3", world.name(), self.name())
        } else if !batch4_ready {
            format!("{}-{}-ingredients-batch4", world.name(), self.name())
        } else if !batch5_ready {
            format!("{}-{}-ingredients-batch5", world.name(), self.name())
        } else if !batch6_ready {
            format!("{}-{}-ingredients-batch6", world.name(), self.name())
        } else {
            format!("{}-{}-ready", world.name(), self.name())
        };
        get_message(&message, world.lang(), None)
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(
            &format!("{}-{}-short", world.name(), self.name()),
            world.lang(),
            None,
        )
    }
}

scene_base!(Garden, "garden", []);
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

scene_base!(ChildrenHouse, "children_house", []);
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

scene_base!(ChildrenGarden, "children_garden", []);
impl Description for ChildrenGarden {
    fn long(&self, world: &dyn World) -> String {
        if world
            .items()
            .values()
            .filter(|e| e.get_tags().contains(&"toy".to_string()))
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

scene_base!(WayHome, "way_home", []);
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
