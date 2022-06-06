use pabitell_lib::{data, Event, ItemState, Narrator, World};
use serde_json::Value;

use crate::events::{self, ProtocolEvent};

fn to_dynamic_event(event: Box<dyn Event>) -> Box<dyn Event> {
    event
}

#[derive(Default, Debug)]
pub struct Doll;

impl Narrator for Doll {
    fn all_events(&self, world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut res: Vec<Box<dyn Event>> = vec![];

        // Talk in home
        for (character, idx) in &[
            ("doggie", 0),
            ("kitie", 1),
            ("doggie", 2),
            ("kitie", 3),
            ("doggie", 4),
            ("kitie", 6),
            ("doggie", 7),
            ("kitie", 8),
            ("doggie", 9),
            ("kitie", 10),
            ("doggie", 11),
            ("kitie", 12),
            ("doggie", 13),
            ("kitie", 14),
        ] {
            res.push(Box::new(events::make_talk(
                "talk_in_home",
                data::TalkData::new(character, "home", *idx),
            )));
        }

        // Move to walk
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_walk",
                data::MoveData::new(character, "walk"),
                "home",
                Some(5),
                None,
                false,
            )));
        }

        // Move to diggie search
        res.push(Box::new(events::make_move(
            "move_to_doggie_search",
            data::MoveData::new("doggie", "doggie_search"),
            "home",
            Some(15),
            None,
            false,
        )));

        // Move to kitie search
        res.push(Box::new(events::make_move(
            "move_to_kitie_search",
            data::MoveData::new("kitie", "kitie_search"),
            "home",
            Some(16),
            None,
            false,
        )));

        // Lay down
        for character in &["doggie", "kitie"] {
            for item in world.items().values().filter_map(|e| {
                if e.get_tags().contains(&format!("{character}_pick")) {
                    Some(e.name())
                } else {
                    None
                }
            }) {
                res.push(Box::new(events::make_lay_down(
                    "lay_down",
                    data::UseItemData::new(character, item),
                )));
            }
        }

        // Go back home from walk
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_home",
                data::MoveData::new(character, "home"),
                "walk",
                Some(7),
                None,
                true,
            )));
        }

        // Talk on walk
        for (character, idx) in &[
            ("doggie", 0),
            ("kitie", 1),
            ("doggie", 2),
            ("kitie", 3),
            ("doggie", 5),
            ("kitie", 6),
        ] {
            res.push(Box::new(events::make_talk(
                "talk_on_walk",
                data::TalkData::new(character, "walk", *idx),
            )));
        }

        // Find the doll
        res.push(Box::new(events::make_find_doll(data::UseItemData::new(
            "kitie", "doll",
        ))));
        res.push(Box::new(events::make_find_doll(data::UseItemData::new(
            "doggie", "doll",
        ))));

        // Go back home from searches
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_home",
                data::MoveData::new(character, "home"),
                &format!("{character}_search"),
                None,
                Some((
                    vec![format!("{character}_pick")],
                    ItemState::Owned(character.to_string()),
                )),
                true,
            )));
        }

        // Make picks
        for character in &["doggie", "kitie"] {
            for toy in world.items().values().filter_map(|e| {
                let tags = e.get_tags();
                if tags.contains(&format!("{character}_pick")) {
                    Some(e.name())
                } else {
                    None
                }
            }) {
                res.push(Box::new(events::make_pick(
                    "pick",
                    data::PickData::new(character, toy),
                )));
            }
        }

        res
    }

    fn parse_event(&self, world: &dyn World, value: Value) -> Option<Box<dyn Event>> {
        let event: Result<ProtocolEvent, serde_json::Error> = serde_json::from_value(value);

        match event {
            Ok(ProtocolEvent::TalkInHome(data)) => Some(to_dynamic_event(Box::new(
                events::make_talk("talk_in_home", data),
            ))),
            Ok(ProtocolEvent::MoveToWalk(data)) => Some(to_dynamic_event(Box::new(
                events::make_move("move_to_walk", data, "home", Some(5), None, true),
            ))),
            Ok(ProtocolEvent::TalkOnWalk(data)) => Some(to_dynamic_event(Box::new(
                events::make_talk("talk_on_walk", data),
            ))),
            Ok(ProtocolEvent::FindDoll(data)) => {
                Some(to_dynamic_event(Box::new(events::make_find_doll(data))))
            }
            Ok(ProtocolEvent::MoveToHome(data)) => {
                let doggie_picked = world
                    .items()
                    .values()
                    .filter(|e| e.get_tags().contains(&"doggie_pick".to_owned()))
                    .all(|e| e.state() != &ItemState::InScene("doggie_search".to_owned()));
                let kitie_picked = world
                    .items()
                    .values()
                    .filter(|e| e.get_tags().contains(&"kitie_pick".to_owned()))
                    .all(|e| e.state() != &ItemState::InScene("kitie_search".to_owned()));
                Some(if kitie_picked {
                    to_dynamic_event(Box::new(events::make_move(
                        "move_to_home",
                        data,
                        "kitie_search",
                        None,
                        Some((
                            vec![format!("kitie_pick")],
                            ItemState::Owned("kitie".to_string()),
                        )),
                        true,
                    )))
                } else if doggie_picked {
                    to_dynamic_event(Box::new(events::make_move(
                        "move_to_home",
                        data,
                        "doggie_search",
                        None,
                        Some((
                            vec![format!("doggie_pick")],
                            ItemState::Owned("doggie".to_string()),
                        )),
                        true,
                    )))
                } else {
                    to_dynamic_event(Box::new(events::make_move(
                        "move_to_home",
                        data,
                        "walk",
                        Some(7),
                        None,
                        true,
                    )))
                })
            }
            Ok(ProtocolEvent::MoveToDoggieSearch(data)) => Some(to_dynamic_event(Box::new(
                events::make_move("move_to_doggie_search", data, "home", Some(15), None, true),
            ))),
            Ok(ProtocolEvent::MoveToKitieSearch(data)) => Some(to_dynamic_event(Box::new(
                events::make_move("move_to_kitie_search", data, "home", Some(16), None, true),
            ))),
            Ok(ProtocolEvent::Pick(data)) => {
                Some(to_dynamic_event(Box::new(events::make_pick("pick", data))))
            }
            Ok(ProtocolEvent::LayDown(data)) => Some(to_dynamic_event(Box::new(
                events::make_lay_down("lay_down", data),
            ))),
            Err(_) => None,
        }
    }
}
