use pabitell_lib::{data, Character, Event, ItemState, Narrator, World};
use serde_json::Value;

use crate::{
    characters,
    events::{self, ProtocolEvent},
};

fn to_dynamic_event(event: Box<dyn Event>) -> Box<dyn Event> {
    event
}

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
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 0),
                    )) as Box<dyn Event>),
                    1 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 1),
                    ))),
                    2 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 2),
                    )) as Box<dyn Event>),
                    3 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 3),
                    )) as Box<dyn Event>),
                    4 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 4),
                    )) as Box<dyn Event>),
                    5 => {
                        ["doggie", "kitie"].iter().for_each(|c| {
                            res.push(Box::new(events::make_move(
                                "move_to_walk",
                                data::MoveData::new(c, "walk"),
                                "home",
                                Some(5),
                                false,
                            )) as Box<dyn Event>);
                        });
                    }
                    6 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 6),
                    )) as Box<dyn Event>),
                    7 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 7),
                    )) as Box<dyn Event>),
                    8 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 8),
                    )) as Box<dyn Event>),
                    9 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 9),
                    )) as Box<dyn Event>),
                    10 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 10),
                    )) as Box<dyn Event>),
                    11 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 11),
                    )) as Box<dyn Event>),
                    12 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 12),
                    )) as Box<dyn Event>),
                    13 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("doggie", "home", 13),
                    )) as Box<dyn Event>),
                    14 => res.push(Box::new(events::make_talk(
                        "talk_in_home",
                        data::TalkData::new("kitie", "home", 14),
                    )) as Box<dyn Event>),
                    15 => {
                        res.push(Box::new(events::make_move(
                            "move_to_doggie_search",
                            data::MoveData::new("doggie", "doggie_search"),
                            "home",
                            Some(15),
                            false,
                        )) as Box<dyn Event>);
                    }
                    16 => {
                        res.push(Box::new(events::make_move(
                            "move_to_kitie_search",
                            data::MoveData::new("kitie", "kitie_search"),
                            "home",
                            Some(16),
                            false,
                        )) as Box<dyn Event>);
                    }
                    17 => {
                        for c in &["doggie", "kitie"] {
                            let items = world
                                .items()
                                .values()
                                .filter(|v| {
                                    v.get_tags().contains(&format!("{}_pick", c))
                                        && v.state() == &ItemState::Owned(c.to_string())
                                })
                                .collect::<Vec<_>>();
                            for item in items {
                                res.push(Box::new(events::make_lay_down(
                                    "lay_down",
                                    data::UseItemData::new(c, item.name()),
                                )) as Box<dyn Event>);
                            }
                        }
                    }
                    18 => {} // final dialog
                    _ => unimplemented!(),
                }
            }
            (Some(d), Some(k)) if d == "walk" && k == "home" => {
                if world.items().get("doll").unwrap().state() == &ItemState::Unassigned {
                    // way back
                    res.push(Box::new(events::make_move(
                        "move_to_home",
                        data::MoveData::new("doggie", "home"),
                        "walk",
                        Some(7),
                        true,
                    )) as Box<dyn Event>);
                } else {
                    res.push(Box::new(events::make_move(
                        "move_to_walk",
                        data::MoveData::new("kitie", "walk"),
                        "home",
                        Some(5),
                        false,
                    )) as Box<dyn Event>);
                }
            }
            (Some(d), Some(k)) if d == "home" && k == "walk" => {
                if world.items().get("doll").unwrap().state() == &ItemState::Unassigned {
                    // way back
                    res.push(Box::new(events::make_move(
                        "move_to_home",
                        data::MoveData::new("kitie", "home"),
                        "walk",
                        Some(7),
                        true,
                    )) as Box<dyn Event>);
                } else {
                    res.push(Box::new(events::make_move(
                        "move_to_walk",
                        data::MoveData::new("doggie", "walk"),
                        "home",
                        Some(5),
                        false,
                    )) as Box<dyn Event>);
                }
            }
            (Some(d), Some(k)) if d == "walk" && k == "walk" => {
                let scene = world.scenes().get("walk").unwrap();
                match scene.dialog().unwrap() {
                    0 => res.push(Box::new(events::make_talk(
                        "talk_on_walk",
                        data::TalkData::new("doggie", "walk", 0),
                    )) as Box<dyn Event>),
                    1 => res.push(Box::new(events::make_talk(
                        "talk_on_walk",
                        data::TalkData::new("kitie", "walk", 1),
                    ))),
                    2 => res.push(Box::new(events::make_talk(
                        "talk_on_walk",
                        data::TalkData::new("doggie", "walk", 2),
                    ))),
                    3 => res.push(Box::new(events::make_talk(
                        "talk_on_walk",
                        data::TalkData::new("kitie", "walk", 3),
                    ))),
                    4 => {
                        res.push(Box::new(events::make_find_doll(data::UseItemData::new(
                            "kitie", "doll",
                        ))));
                        res.push(Box::new(events::make_find_doll(data::UseItemData::new(
                            "doggie", "doll",
                        ))));
                    }
                    5 => res.push(Box::new(events::make_talk(
                        "talk_on_walk",
                        data::TalkData::new("doggie", "walk", 5),
                    ))),
                    6 => res.push(Box::new(events::make_talk(
                        "talk_on_walk",
                        data::TalkData::new("kitie", "walk", 6),
                    ))),
                    7 => {
                        ["doggie", "kitie"].iter().for_each(|c| {
                            res.push(Box::new(events::make_move(
                                "move_to_home",
                                data::MoveData::new(c, "home"),
                                "walk",
                                Some(7),
                                true,
                            )) as Box<dyn Event>);
                        });
                    }
                    _ => unimplemented!(),
                }
            }
            (Some(d), Some(_)) if d == "doggie_search" => {
                let mut items = world
                    .items()
                    .values()
                    .filter(|v| {
                        v.get_tags().contains(&"doggie_pick".to_owned())
                            && v.state() == &ItemState::InScene("doggie_search".to_string())
                    })
                    .collect::<Vec<_>>();
                if items.is_empty() {
                    res.push(Box::new(events::make_move(
                        "move_to_home",
                        data::MoveData::new("doggie", "home"),
                        "doggie_search",
                        None,
                        true,
                    )) as Box<dyn Event>);
                } else {
                    for item in items {
                        res.push(Box::new(events::make_pick(
                            "pick",
                            data::PickData::new("doggie", item.name()),
                        )) as Box<dyn Event>);
                    }
                }
            }
            (Some(_), Some(k)) if k == "kitie_search" => {
                let items = world
                    .items()
                    .values()
                    .filter(|v| {
                        v.get_tags().contains(&"kitie_pick".to_owned())
                            && v.state() == &ItemState::InScene("kitie_search".to_string())
                    })
                    .collect::<Vec<_>>();
                if items.is_empty() {
                    res.push(Box::new(events::make_move(
                        "move_to_home",
                        data::MoveData::new("kitie", "home"),
                        "kitie_search",
                        None,
                        true,
                    )) as Box<dyn Event>);
                } else {
                    for item in items {
                        res.push(Box::new(events::make_pick(
                            "pick",
                            data::PickData::new("kitie", item.name()),
                        )) as Box<dyn Event>);
                    }
                }
            }
            _ => unreachable!(),
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
                events::make_move("move_to_walk", data, "home", Some(5), true),
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
                        true,
                    )))
                } else if doggie_picked {
                    to_dynamic_event(Box::new(events::make_move(
                        "move_to_home",
                        data,
                        "doggie_search",
                        None,
                        true,
                    )))
                } else {
                    to_dynamic_event(Box::new(events::make_move(
                        "move_to_home",
                        data,
                        "walk",
                        Some(7),
                        true,
                    )))
                })
            }
            Ok(ProtocolEvent::MoveToDoggieSearch(data)) => Some(to_dynamic_event(Box::new(
                events::make_move("move_to_doggie_search", data, "home", Some(15), true),
            ))),
            Ok(ProtocolEvent::MoveToKitieSearch(data)) => Some(to_dynamic_event(Box::new(
                events::make_move("move_to_kitie_search", data, "home", Some(16), true),
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
