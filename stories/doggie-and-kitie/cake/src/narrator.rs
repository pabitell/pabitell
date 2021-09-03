use pabitell_lib::{
    data, translations::get_available_locales, Character, Description, Event, Id, Item, ItemState,
    Named, Narrator, Scene, World, WorldBuilder,
};

use crate::{characters, events, CakeWorld};

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
                                    let event: Box<dyn Event> =
                                        Box::new(events::make_disliked_pick(data::VoidData::new(
                                            "pick_disliked_ingredient",
                                            character,
                                            Some(e.name()),
                                        )));
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
                    .filter(|e| {
                        e.get_tags().contains(&"ingredient".to_string())
                            && e.get_tags().contains(&"accepted".to_string())
                    })
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
}
