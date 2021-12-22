use pabitell_lib::{
    conditions, data, translations::get_available_locales, Character, Description, Event, Id, Item,
    ItemState, Named, Narrator, Scene, World, WorldBuilder,
};
use serde_json::Value;

use crate::{characters, events, world::DollWorld};

#[derive(Default, Debug)]
pub struct Doll;

impl Narrator for Doll {
    fn available_events(&self, world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut res = vec![];
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

        match (doggie.scene().as_ref(), kitie.scene().as_ref()) {
            (Some(d), Some(k)) if d == "home" && k == "home" => {
                let scene = world.scenes().get("home").unwrap();
                match scene.dialog().unwrap() {
                    0 => res.push(Box::new(events::make_talk(
                        data::VoidData::new("talk_in_home", "doggie", None as Option<String>),
                        "home",
                        &["doggie", "kitie"],
                        0,
                    )) as Box<dyn Event>),
                    1 => res.push(Box::new(events::make_talk(
                        data::VoidData::new("talk_in_home", "kitie", None as Option<String>),
                        "home",
                        &["doggie", "kitie"],
                        1,
                    ))),
                    2 => res.push(Box::new(events::make_talk(
                        data::VoidData::new("talk_in_home", "doggie", None as Option<String>),
                        "home",
                        &["doggie", "kitie"],
                        2,
                    )) as Box<dyn Event>),
                    3 => res.push(Box::new(events::make_talk(
                        data::VoidData::new("talk_in_home", "kitie", None as Option<String>),
                        "home",
                        &["doggie", "kitie"],
                        3,
                    ))),
                    4 => res.push(Box::new(events::make_talk(
                        data::VoidData::new("talk_in_home", "doggie", None as Option<String>),
                        "home",
                        &["doggie", "kitie"],
                        4,
                    )) as Box<dyn Event>),
                    5 => {
                        ["doggie", "kitie"].iter().for_each(|c| {
                            res.push(Box::new(events::make_move(
                                data::MoveData::new("move_to_walk", c, "walk"),
                                c,
                                "home",
                                5,
                            )) as Box<dyn Event>);
                        });
                    }
                    _ => unimplemented!(),
                }
            }
            (Some(d), Some(k)) if d == "walk" && k == "home" => {
                res.push(Box::new(events::make_move(
                    data::MoveData::new("move_to_walk", "kitie", "walk"),
                    "kitie",
                    "home",
                    5,
                )) as Box<dyn Event>);
            }
            (Some(d), Some(k)) if d == "home" && k == "walk" => {
                res.push(Box::new(events::make_move(
                    data::MoveData::new("move_to_walk", "doggie", "walk"),
                    "doggie",
                    "home",
                    5,
                )) as Box<dyn Event>);
            }
            _ => unreachable!(),
        }

        res
    }

    fn parse_event(&self, value: &Value) -> Option<Box<dyn Event>> {
        // TODO validate characters, items, scenes
        match &value["name"] {
            Value::String(name) if name == "talk_in_home" => {
                /*
                if let Value::String(character) = &value["character"] {
                    let data = data::MoveData::new(name, character, "kitchen");
                    Some(Box::new(events::make_move_to_kitchen(data)))
                } else {
                    None
                }
                */
                None
            }
            Value::String(name) if name == "move_to_walk" => {
                /*
                 */
                None
            }
            _ => None,
        }
    }
}
