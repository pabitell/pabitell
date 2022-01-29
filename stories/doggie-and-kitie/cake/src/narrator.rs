use pabitell_lib::{conditions, data, Character, Event, ItemState, Narrator, World};
use serde_json::Value;

use crate::{characters, events};

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
            }
            (Some(d), Some(k)) if d == "playground" && k == "kitchen" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                    data::MoveData::new("move_to_kitchen", "doggie", "kitchen"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "playground" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen(
                    data::MoveData::new("move_to_kitchen", "kitie", "kitchen"),
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
                                        data::PickData::new("pick_ingredient", character, e.name()),
                                        false,
                                    ));
                                    res.push(event);
                                }
                            } else if e.get_tags().contains(&"rejected".to_string()) {
                                for character in ["doggie", "kitie"] {
                                    let event: Box<dyn Event> = Box::new(events::make_pick(
                                        data::PickData::new(
                                            "pick_disliked_ingredient",
                                            character,
                                            e.name(),
                                        ),
                                        true,
                                    ));
                                    res.push(event);
                                }
                            }
                        }
                    }
                    ItemState::Owned(character) => {
                        let event = Box::new(events::make_use_item(
                            data::UseItemData::new("add_ingredient", character, e.name()),
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
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_children_garden(data::MoveData::new(
                                "move_to_children_garden",
                                character,
                                "children_garden",
                            )));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "children_garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_garden(
                    data::MoveData::new("move_to_children_garden", "doggie", "children_garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "kitchen" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_garden(
                    data::MoveData::new("move_to_children_garden", "kitie", "children_garden"),
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
                                data::PickData::new("play", character, e.name()),
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
                            data::MoveData::new("move_to_garden", character, "garden"),
                        ));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_garden(
                    data::MoveData::new("move_to_garden", "doggie", "garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_garden(
                    data::MoveData::new("move_to_garden", "kitie", "garden"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "garden" => {
                if world.items().get("bad_dog").unwrap().state()
                    == &ItemState::InScene("garden".into())
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> = Box::new(events::make_find_bad_dog(
                            data::PickData::new("find", character, "bad_dog"),
                        ));
                        res.push(event);
                    }
                } else {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_children_house(data::MoveData::new(
                                "move_to_children_house",
                                character,
                                "children_house",
                            )));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_house" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_house(
                    data::MoveData::new("move_to_children_house", "doggie", "children_house"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_house(
                    data::MoveData::new("move_to_children_house", "kitie", "children_house"),
                ));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "children_house" => {
                if !doggie.consumed_pie {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "doggie", Some("pie")),
                    ));
                    res.push(event);
                }
                if !doggie.consumed_soup {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "doggie", Some("soup")),
                    ));
                    res.push(event);
                }
                if !doggie.consumed_dumplings {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "doggie", Some("dumplings")),
                    ));
                    res.push(event);
                }
                if !doggie.consumed_meat {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "doggie", Some("meat")),
                    ));
                    res.push(event);
                }

                if !kitie.consumed_pie {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "kitie", Some("pie")),
                    ));
                    res.push(event);
                }
                if !kitie.consumed_soup {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "kitie", Some("soup")),
                    ));
                    res.push(event);
                }
                if !kitie.consumed_dumplings {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "kitie", Some("dumplings")),
                    ));
                    res.push(event);
                }
                if !kitie.consumed_meat {
                    let event: Box<dyn Event> = Box::new(events::make_eat_meal(
                        data::VoidData::new("eat", "kitie", Some("meat")),
                    ));
                    res.push(event);
                }
            }
            (Some(d), Some(k)) if d == "way_home" && k == "way_home" => {}
            _ => unreachable!(),
        }

        res
    }

    fn parse_event(&self, _world: &dyn World, value: &Value) -> Option<Box<dyn Event>> {
        // TODO validate characters, items, scenes
        match &value["name"] {
            Value::String(name) if name == "move_to_kitchen" => {
                if let Value::String(character) = &value["character"] {
                    let data = data::MoveData::new(name, character, "kitchen");
                    Some(Box::new(events::make_move_to_kitchen(data)))
                } else {
                    None
                }
            }
            Value::String(name) if name == "move_to_children_garden" => {
                if let Value::String(character) = &value["character"] {
                    let data = data::MoveData::new(name, character, "children_garden");
                    Some(Box::new(events::make_move_to_children_garden(data)))
                } else {
                    None
                }
            }
            Value::String(name) if name == "move_to_garden" => {
                if let Value::String(character) = &value["character"] {
                    let data = data::MoveData::new(name, character, "garden");
                    Some(Box::new(events::make_move_to_garden(data)))
                } else {
                    None
                }
            }
            Value::String(name) if name == "move_to_children_house" => {
                if let Value::String(character) = &value["character"] {
                    let data = data::MoveData::new(name, character, "children_house");
                    Some(Box::new(events::make_move_to_children_house(data)))
                } else {
                    None
                }
            }
            Value::String(name) if ["pick_ingredient", "pick"].contains(&name.as_str()) => {
                if let Value::String(character) = &value["character"] {
                    if let Value::String(item) = &value["item"] {
                        let data = data::PickData::new(name, character, item);
                        Some(Box::new(events::make_pick(data, false)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Value::String(name) if name == "give_sand_cake" => {
                if let Value::String(item) = &value["item"] {
                    if let Value::String(from_character) = &value["from_character"] {
                        if let Value::String(to_character) = &value["to_character"] {
                            let data =
                                data::GiveData::new(name, from_character, to_character, item);
                            Some(Box::new(events::make_give_sand_cake(data)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Value::String(name) if name == "play" => {
                if let Value::String(item) = &value["item"] {
                    if let Value::String(character) = &value["character"] {
                        let data = data::PickData::new(name, character, item);
                        Some(Box::new(events::make_pick(data, true)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Value::String(name) if name == "find" => {
                if let Value::String(character) = &value["character"] {
                    let data = data::PickData::new(name, character, "bad_dog");
                    Some(Box::new(events::make_find_bad_dog(data)))
                } else {
                    None
                }
            }
            Value::String(name) if name == "eat" => {
                if let Value::String(item) = &value["item"] {
                    if let Value::String(character) = &value["character"] {
                        Some(Box::new(events::make_eat_meal(data::VoidData::new(
                            name,
                            character,
                            Some(item),
                        ))))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Value::String(name) if name == "pick_disliked_ingredient" => {
                if let Value::String(character) = &value["character"] {
                    if let Value::String(item) = &value["item"] {
                        let data = data::PickData::new(name, character, item);
                        Some(Box::new(events::make_pick(data, true)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Value::String(name) if name == "add_ingredient" => {
                if let Value::String(character) = &value["character"] {
                    if let Value::String(item) = &value["item"] {
                        let data = data::UseItemData::new(name, character, item);
                        Some(Box::new(events::make_use_item(data, true)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
