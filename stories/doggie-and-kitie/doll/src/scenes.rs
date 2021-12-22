use pabitell_lib::{conditions, scene_base, AsAny, Description, Named, World};
use std::any::Any;

use crate::{characters, translations::get_message};

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
