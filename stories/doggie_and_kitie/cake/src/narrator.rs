use pabitell_lib::{conditions, data, Character, Event, ItemState, Narrator, World};
use serde_json::Value;

use crate::{
    characters,
    events::{self, ProtocolEvent},
};

fn to_dynamic_event(event: Box<dyn Event>) -> Box<dyn Event> {
    event
}

#[derive(Default, Debug)]
pub struct Cake;

impl Narrator for Cake {
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
            (Some(d), Some(k)) if d == "playground" && k == "playground" => {
                let sand_cake = world.items().get("sand_cake").unwrap();
                match sand_cake.state() {
                    ItemState::Unassigned => {
                        let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                            data::MoveData::new("doggie", "kitchen"),
                        ));
                        res.push(event);
                        let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                            data::MoveData::new("kitie", "kitchen"),
                        ));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "doggie" => {
                        let event: Box<dyn Event> = Box::new(events::make_give_sand_cake(
                            data::GiveData::new("doggie", "kitie", "sand_cake"),
                        ));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "kitie" => {
                        let event: Box<dyn Event> = Box::new(events::make_give_sand_cake(
                            data::GiveData::new("kitie", "doggie", "sand_cake"),
                        ));
                        res.push(event);
                    }
                    ItemState::InScene(e) if e == "playground" => {
                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            "pick",
                            data::PickData::new("kitie", "sand_cake"),
                            false,
                        ));
                        res.push(event);

                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            "pick",
                            data::PickData::new("doggie", "sand_cake"),
                            false,
                        ));
                        res.push(event);
                    }
                    _ => {}
                }
            }
            (Some(d), Some(k)) if d == "playground" && k == "kitchen" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                    data::MoveData::new("doggie", "kitchen"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "playground" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                    data::MoveData::new("kitie", "kitchen"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "kitchen" => {
                // Pick ingredient
                world.items().values().for_each(|e| match e.state() {
                    ItemState::InScene(p) if p == "kitchen" => {
                        if e.get_tags().contains(&"ingredient".to_string()) {
                            if e.get_tags().contains(&"batch2".to_string()) {
                                // All batch1 items done
                                if !conditions::all_items_with_tags_in_state(
                                    world,
                                    &["batch1".to_string()],
                                    ItemState::Unassigned,
                                ) {
                                    return;
                                }
                            } else if e.get_tags().contains(&"batch3".to_string()) {
                                // All batch1, batch2 items done
                                if !conditions::all_items_with_tags_in_state(
                                    world,
                                    &["batch1".to_string(), "batch2".to_string()],
                                    ItemState::Unassigned,
                                ) {
                                    return;
                                }
                            } else if e.get_tags().contains(&"batch4".to_string()) {
                                // All batch1, batch2, batch3 items done
                                if !conditions::all_items_with_tags_in_state(
                                    world,
                                    &[
                                        "batch1".to_string(),
                                        "batch2".to_string(),
                                        "batch3".to_string(),
                                    ],
                                    ItemState::Unassigned,
                                ) {
                                    return;
                                }
                            } else if e.get_tags().contains(&"batch5".to_string()) {
                                // All batch1, batch2, batch3, batch4 items done
                                if !conditions::all_items_with_tags_in_state(
                                    world,
                                    &[
                                        "batch1".to_string(),
                                        "batch2".to_string(),
                                        "batch3".to_string(),
                                        "batch4".to_string(),
                                    ],
                                    ItemState::Unassigned,
                                ) {
                                    return;
                                }
                            } else if e.get_tags().contains(&"batch6".to_string()) {
                                // All batch1, batch2, batch3, batch4, batch5 items done
                                if !conditions::all_items_with_tags_in_state(
                                    world,
                                    &[
                                        "batch1".to_string(),
                                        "batch2".to_string(),
                                        "batch3".to_string(),
                                        "batch4".to_string(),
                                        "batch5".to_string(),
                                    ],
                                    ItemState::Unassigned,
                                ) {
                                    return;
                                }
                            }
                            if e.get_tags().contains(&"accepted".to_string()) {
                                for character in ["doggie", "kitie"] {
                                    let event: Box<dyn Event> = Box::new(events::make_pick(
                                        "pick_ingredient",
                                        data::PickData::new(character, e.name()),
                                        false,
                                    ));
                                    res.push(event);
                                }
                            } else if e.get_tags().contains(&"rejected".to_string()) {
                                for character in ["doggie", "kitie"] {
                                    let event: Box<dyn Event> = Box::new(events::make_pick(
                                        "pick_disliked_ingredient",
                                        data::PickData::new(character, e.name()),
                                        true,
                                    ));
                                    res.push(event);
                                }
                            }
                        }
                    }
                    ItemState::Owned(character) => {
                        let event = Box::new(events::make_use_item(
                            "add_ingredient",
                            data::UseItemData::new(character, e.name()),
                            true,
                        ));
                        res.push(event);
                    }
                    _ => {}
                });

                // Move events
                if world
                    .items()
                    .values()
                    .filter(|e| e.get_tags().contains(&"ingredient".to_string()))
                    .all(|e| e.state() == &ItemState::Unassigned)
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> = Box::new(events::make_move_to_children_garden(
                            data::MoveData::new(character, "children_garden"),
                        ));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "children_garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_garden(
                    data::MoveData::new("doggie", "children_garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "kitchen" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_garden(
                    data::MoveData::new("kitie", "children_garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "children_garden" => {
                // make picks
                world
                    .items()
                    .values()
                    .filter(|e| {
                        e.get_tags().contains(&"toy".to_string())
                            && e.state() == &ItemState::InScene("children_garden".into())
                    })
                    .for_each(|e| {
                        for character in ["doggie", "kitie"] {
                            let event: Box<dyn Event> = Box::new(events::make_pick(
                                "play",
                                data::PickData::new(character, e.name()),
                                true,
                            ));
                            res.push(event);
                        }
                    });

                // make move events
                if world
                    .items()
                    .values()
                    .filter(|e| e.get_tags().contains(&"toy".to_string()))
                    .all(|e| e.state() == &ItemState::Unassigned)
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> = Box::new(events::make_move_to_garden(
                            data::MoveData::new(character, "garden"),
                        ));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_garden(
                    data::MoveData::new("doggie", "garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_garden(
                    data::MoveData::new("kitie", "garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "garden" => {
                if world.items().get("bad_dog").unwrap().state()
                    == &ItemState::InScene("garden".into())
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> = Box::new(events::make_find_bad_dog(
                            data::PickData::new(character, "bad_dog"),
                        ));
                        res.push(event);
                    }
                } else {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> = Box::new(events::make_move_to_children_house(
                            data::MoveData::new(character, "children_house"),
                        ));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_house" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_house(
                    data::MoveData::new("doggie", "children_house"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_house(
                    data::MoveData::new("kitie", "children_house"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "children_house" => {
                if !doggie.consumed_pie {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("doggie", Some("pie")),
                    ));
                    res.push(event);
                }
                if !doggie.consumed_soup {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("doggie", Some("soup")),
                    ));
                    res.push(event);
                }
                if !doggie.consumed_dumplings {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("doggie", Some("dumplings")),
                    ));
                    res.push(event);
                }
                if !doggie.consumed_meat {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("doggie", Some("meat")),
                    ));
                    res.push(event);
                }

                if !kitie.consumed_pie {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("kitie", Some("pie")),
                    ));
                    res.push(event);
                }
                if !kitie.consumed_soup {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("kitie", Some("soup")),
                    ));
                    res.push(event);
                }
                if !kitie.consumed_dumplings {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("kitie", Some("dumplings")),
                    ));
                    res.push(event);
                }
                if !kitie.consumed_meat {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("kitie", Some("meat")),
                    ));
                    res.push(event);
                }
            }
            (Some(d), Some(k)) if d == "way_home" && k == "way_home" => {}
            _ => unreachable!(),
        }

        res
    }

    fn parse_event(&self, _world: &dyn World, value: Value) -> Option<Box<dyn Event>> {
        let event: Result<ProtocolEvent, serde_json::Error> = serde_json::from_value(value);

        match event {
            Ok(ProtocolEvent::MoveToKitchen(data)) => Some(to_dynamic_event(Box::new(
                events::make_move_to_kitchen(data),
            ))),
            Ok(ProtocolEvent::MoveToChildrenGarden(data)) => Some(to_dynamic_event(Box::new(
                events::make_move_to_children_garden(data),
            ))),
            Ok(ProtocolEvent::MoveToGarden(data)) => Some(to_dynamic_event(Box::new(
                events::make_move_to_garden(data),
            ))),
            Ok(ProtocolEvent::MoveToChildrenHouse(data)) => Some(to_dynamic_event(Box::new(
                events::make_move_to_children_house(data),
            ))),
            Ok(ProtocolEvent::PickIngredient(data)) => Some(to_dynamic_event(Box::new(
                events::make_pick("pick_ingredient", data, false),
            ))),
            Ok(ProtocolEvent::Pick(data)) => Some(to_dynamic_event(Box::new(events::make_pick(
                "pick", data, false,
            )))),
            Ok(ProtocolEvent::GiveSandCake(data)) => Some(to_dynamic_event(Box::new(
                events::make_give_sand_cake(data),
            ))),
            Ok(ProtocolEvent::Give(data)) => {
                Some(to_dynamic_event(Box::new(events::make_give(data))))
            }
            Ok(ProtocolEvent::Play(data)) => Some(to_dynamic_event(Box::new(events::make_pick(
                "play", data, true,
            )))),
            Ok(ProtocolEvent::FindBadDog(data)) => {
                Some(to_dynamic_event(Box::new(events::make_find_bad_dog(data))))
            }
            Ok(ProtocolEvent::Eat(data)) => {
                Some(to_dynamic_event(Box::new(events::make_eat_meal(data))))
            }
            Ok(ProtocolEvent::PickDislikedIngredient(data)) => Some(to_dynamic_event(Box::new(
                events::make_pick("pick_disliked_ingredient", data, true),
            ))),
            Ok(ProtocolEvent::AddIngredient(data)) => Some(to_dynamic_event(Box::new(
                events::make_use_item("add_ingredient", data, true),
            ))),
            Err(_) => None,
        }
    }
}
