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
                /*
                let sand_cake = world.items().get("sand_cake").unwrap();
                match sand_cake.state() {
                    ItemState::Unassigned => {
                        let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                            data::MoveData::new("move_to_kitchen", "doggie", "kitchen"),
                        ));
                        res.push(event);
                        let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                            data::MoveData::new("move_to_kitchen", "kitie", "kitchen"),
                        ));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "doggie" => {
                        let event: Box<dyn Event> = Box::new(events::make_give_sand_cake(
                            data::GiveData::new("give_sand_cake", "doggie", "kitie", "sand_cake"),
                        ));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "kitie" => {
                        let event: Box<dyn Event> = Box::new(events::make_give_sand_cake(
                            data::GiveData::new("give_sand_cake", "kitie", "doggie", "sand_cake"),
                        ));
                        res.push(event);
                    }
                    ItemState::InScene(e) if e == "playground" => {
                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            data::PickData::new("pick", "kitie", "sand_cake"),
                            false,
                        ));
                        res.push(event);

                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            data::PickData::new("pick", "doggie", "sand_cake"),
                            false,
                        ));
                        res.push(event);
                    }
                    _ => {}
                }
                */
            }
            (Some(d), Some(k)) if d == "walk" && k == "home" => {
                /*
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                    data::MoveData::new("move_to_kitchen", "doggie", "kitchen"),
                ));
                res.push(event);
                */
            }
            (Some(d), Some(k)) if d == "home" && k == "walk" => {
                /*
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                    data::MoveData::new("move_to_kitchen", "kitie", "kitchen"),
                ));
                res.push(event);
                */
            }
            _ => unreachable!(),
        }

        res
    }

    fn parse_event(&self, value: &Value) -> Option<Box<dyn Event>> {
        // TODO validate characters, items, scenes
        match &value["name"] {
            Value::String(name) if name == "go_for_a_walk" => {
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
            _ => None,
        }
    }
}
