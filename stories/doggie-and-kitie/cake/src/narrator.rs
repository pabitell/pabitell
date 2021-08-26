use pabitell_lib::{
    data, translations::get_available_locales, Character, Description, Event, Id, Item, ItemState,
    Named, Narrator, Scene, World, WorldBuilder,
};
use uuid::Uuid;

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
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_kitchen(data::MoveData::new(
                                Uuid::default(),
                                "move_to_kitchen".to_string(),
                                "doggie".to_string(),
                                "kitchen".to_string(),
                            )));
                        res.push(event);
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_kitchen(data::MoveData::new(
                                Uuid::default(),
                                "move_to_kitchen".to_string(),
                                "kitie".to_string(),
                                "kitchen".to_string(),
                            )));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "doggie" => {
                        let event: Box<dyn Event> =
                            Box::new(events::make_give_sand_cake(data::GiveData::new(
                                Uuid::default(),
                                "give_sand_cake".to_string(),
                                "doggie".into(),
                                "kitie".into(),
                                "sand_cake".into(),
                            )));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "kitie" => {
                        let event: Box<dyn Event> =
                            Box::new(events::make_give_sand_cake(data::GiveData::new(
                                Uuid::default(),
                                "give_sand_cake".to_string(),
                                "kitie".into(),
                                "doggie".into(),
                                "sand_cake".into(),
                            )));
                        res.push(event);
                    }
                    ItemState::InScene(e) if e == "playground" => {
                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            data::PickData::new(
                                Uuid::default(),
                                "pick".into(),
                                "kitie".to_string(),
                                "sand_cake".to_string(),
                            ),
                            false,
                        ));
                        res.push(event);

                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            data::PickData::new(
                                Uuid::default(),
                                "pick".into(),
                                "doggie".to_string(),
                                "sand_cake".to_string(),
                            ),
                            false,
                        ));
                        res.push(event);
                    }
                    _ => {}
                }
            }
            (Some(d), Some(k)) if d == "playground" && k == "kitchen" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_kitchen(data::MoveData::new(
                        Uuid::default(),
                        "move_to_kitchen".to_string(),
                        "doggie".to_string(),
                        "kitchen".to_string(),
                    )));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "playground" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_kitchen(data::MoveData::new(
                        Uuid::default(),
                        "move_to_kitchen".to_string(),
                        "kitie".to_string(),
                        "kitchen".to_string(),
                    )));
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
                                        data::PickData::new(
                                            Uuid::default(),
                                            "pick_ingredient".into(),
                                            character.to_string(),
                                            e.name().to_string(),
                                        ),
                                        false,
                                    ));
                                    res.push(event);
                                }
                            } else if e.get_tags().contains(&"rejected".to_string()) {
                                for character in ["doggie", "kitie"] {
                                    let event: Box<dyn Event> =
                                        Box::new(events::make_disliked_pick(data::VoidData::new(
                                            Uuid::default(),
                                            "pick_disliked_ingredient".into(),
                                            character.to_string(),
                                            Some(e.name().to_string()),
                                        )));
                                    res.push(event);
                                }
                            }
                        }
                    }
                    ItemState::Owned(character) => {
                        let event = Box::new(events::make_use_item(
                            data::UseItemData::new(
                                Uuid::default(),
                                "add_ingredient".into(),
                                character.to_string(),
                                e.name().to_string(),
                            ),
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
                                Uuid::default(),
                                "move_to_children_garden".to_string(),
                                character.to_string(),
                                "children_garden".to_string(),
                            )));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "children_garden" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_children_garden(data::MoveData::new(
                        Uuid::default(),
                        "move_to_children_garden".to_string(),
                        "doggie".to_string(),
                        "children_garden".to_string(),
                    )));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "kitchen" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_children_garden(data::MoveData::new(
                        Uuid::default(),
                        "move_to_children_garden".to_string(),
                        "kitie".to_string(),
                        "children_garden".to_string(),
                    )));
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
                                data::PickData::new(
                                    Uuid::default(),
                                    "play".into(),
                                    character.to_string(),
                                    e.name().to_string(),
                                ),
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
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_garden(data::MoveData::new(
                                Uuid::default(),
                                "move_to_garden".to_string(),
                                character.to_string(),
                                "garden".to_string(),
                            )));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "garden" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_garden(data::MoveData::new(
                        Uuid::default(),
                        "move_to_garden".to_string(),
                        "doggie".to_string(),
                        "garden".to_string(),
                    )));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_garden" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_garden(data::MoveData::new(
                        Uuid::default(),
                        "move_to_garden".to_string(),
                        "kitie".to_string(),
                        "garden".to_string(),
                    )));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "garden" => {
                if world.items().get("bad_dog").unwrap().state()
                    == &ItemState::InScene("garden".into())
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> =
                            Box::new(events::make_find_bad_dog(data::PickData::new(
                                Uuid::default(),
                                "find".to_string(),
                                character.to_string(),
                                "bad_dog".to_string(),
                            )));
                        res.push(event);
                    }
                } else {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_children_house(data::MoveData::new(
                                Uuid::default(),
                                "move_to_children_house".to_string(),
                                character.to_string(),
                                "children_house".to_string(),
                            )));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_house" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_children_house(data::MoveData::new(
                        Uuid::default(),
                        "move_to_children_house".to_string(),
                        "doggie".to_string(),
                        "children_house".to_string(),
                    )));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "garden" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_children_house(data::MoveData::new(
                        Uuid::default(),
                        "move_to_children_house".to_string(),
                        "kitie".to_string(),
                        "children_house".to_string(),
                    )));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "children_house" => {
                if !doggie.consumed_pie {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "doggie".into(),
                            Some("pie".into()),
                        )));
                    res.push(event);
                }
                if !doggie.consumed_soup {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "doggie".into(),
                            Some("soup".into()),
                        )));
                    res.push(event);
                }
                if !doggie.consumed_dumplings {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "doggie".into(),
                            Some("dumplings".into()),
                        )));
                    res.push(event);
                }
                if !doggie.consumed_meat {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "doggie".into(),
                            Some("meat".into()),
                        )));
                    res.push(event);
                }

                if !kitie.consumed_pie {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "kitie".into(),
                            Some("pie".into()),
                        )));
                    res.push(event);
                }
                if !kitie.consumed_soup {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "kitie".into(),
                            Some("soup".into()),
                        )));
                    res.push(event);
                }
                if !kitie.consumed_dumplings {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "kitie".into(),
                            Some("dumplings".into()),
                        )));
                    res.push(event);
                }
                if !kitie.consumed_meat {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal(data::VoidData::new(
                            Uuid::default(),
                            "eat".into(),
                            "kitie".into(),
                            Some("meat".into()),
                        )));
                    res.push(event);
                }
            }
            (Some(d), Some(k)) if d == "way_home" && k == "way_home" => {}
            _ => unreachable!(),
        }

        res
    }
}
