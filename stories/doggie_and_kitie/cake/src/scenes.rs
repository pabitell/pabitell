use pabitell_lib::{
    conditions, scene_base, scene_no_music, AsAny, Description, ItemState, Music, Named, World,
};
use std::any::Any;

use crate::{characters, translations::get_message};

scene_base!(PlayGround, "playground", []);
scene_no_music!(PlayGround);

impl Description for PlayGround {
    fn long(&self, world: &dyn World) -> String {
        match world.items().get("sand_cake").unwrap().state() {
            ItemState::Owned(character) => match character.as_str() {
                "doggie" => world.get_message(
                    &format!("{}-{}-doggie_pick", world.name(), self.name()),
                    None,
                ),
                "kitie" => world.get_message(
                    &format!("{}-{}-kitie_pick", world.name(), self.name()),
                    None,
                ),
                _ => unreachable!(),
            },
            ItemState::InScene(_) => {
                world.get_message(&format!("{}-{}-initial", world.name(), self.name()), None)
            }
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
                    return world.get_message(
                        &format!("{}-{}-doggie_last", world.name(), self.name()),
                        None,
                    );
                }

                if kitie.sand_cake_last {
                    return world.get_message(
                        &format!("{}-{}-kitie_last", world.name(), self.name()),
                        None,
                    );
                }

                unreachable!()
            }
        }
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}

scene_base!(Kitchen, "kitchen", []);
scene_no_music!(Kitchen);

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
                "batch6".to_string(),
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
        world.get_message(&message, None)
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}

scene_base!(Garden, "garden", []);
scene_no_music!(Garden);

impl Description for Garden {
    fn long(&self, world: &dyn World) -> String {
        if world.items().get("bad_dog").unwrap().state() == &ItemState::Unassigned {
            world.get_message(&format!("{}-{}-found", world.name(), self.name()), None)
        } else {
            world.get_message(&format!("{}-{}-searching", world.name(), self.name()), None)
        }
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}

scene_base!(ChildrenHouse, "children_house", []);
scene_no_music!(ChildrenHouse);

impl Description for ChildrenHouse {
    fn long(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-eating", world.name(), self.name()), None)
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}

scene_base!(ChildrenGarden, "children_garden", []);
scene_no_music!(ChildrenGarden);

impl Description for ChildrenGarden {
    fn long(&self, world: &dyn World) -> String {
        if world
            .items()
            .values()
            .filter(|e| e.get_tags().contains(&"toy".to_string()))
            .all(|e| e.state() == &ItemState::Unassigned)
        {
            world.get_message(&format!("{}-{}-leaving", world.name(), self.name()), None)
        } else {
            world.get_message(&format!("{}-{}-playing", world.name(), self.name()), None)
        }
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}

scene_base!(WayHome, "way_home", []);
scene_no_music!(WayHome);

impl Description for WayHome {
    fn long(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-end", world.name(), self.name()), None)
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}
