use pabitell_lib::{
    translations::get_available_locales, Character, Description, Event, Id, Item, ItemState, Named,
    Narrator, Scene, World, WorldBuilder,
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
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_kitchen("doggie"));
                        res.push(event);
                        let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen("kitie"));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "doggie" => {
                        let event: Box<dyn Event> =
                            Box::new(events::make_give_sand_cake("doggie".into(), "kitie".into()));
                        res.push(event);
                    }
                    ItemState::Owned(e) if e == "kitie" => {
                        let event: Box<dyn Event> =
                            Box::new(events::make_give_sand_cake("kitie".into(), "doggie".into()));
                        res.push(event);
                    }
                    ItemState::InScene(e) if e == "playground" => {
                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            "pick".into(),
                            "kitie",
                            "sand_cake",
                            false,
                        ));
                        res.push(event);

                        let event: Box<dyn Event> = Box::new(events::make_pick(
                            "pick".into(),
                            "doggie",
                            "sand_cake",
                            false,
                        ));
                        res.push(event);
                    }
                    _ => {}
                }
            }
            (Some(d), Some(k)) if d == "playground" && k == "kitchen" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen("doggie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "playground" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_kitchen("kitie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "kitchen" => {
                // Pick ingredient
                world.items().values().for_each(|e| match e.state() {
                    ItemState::InScene(p) if p == "kitchen" => {
                        if e.roles().contains(&"ingredient") {
                            if e.roles().contains(&"accepted") {
                                for character in ["doggie", "kitie"] {
                                    let event: Box<dyn Event> = Box::new(events::make_pick(
                                        "pick_ingredient".into(),
                                        character,
                                        e.name(),
                                        false,
                                    ));
                                    res.push(event);
                                }
                            } else if e.roles().contains(&"rejected") {
                                for character in ["doggie", "kitie"] {
                                    let event: Box<dyn Event> =
                                        Box::new(events::make_disliked_pick(
                                            "pick_disliked_ingredient".into(),
                                            character,
                                            e.name(),
                                        ));
                                    res.push(event);
                                }
                            }
                        }
                    }
                    ItemState::Owned(character) => {
                        let event = Box::new(events::make_use_item(
                            "add_ingredient".into(),
                            character,
                            e.name(),
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
                        e.roles().contains(&"ingredient") && e.roles().contains(&"accepted")
                    })
                    .all(|e| e.state() == &ItemState::Unassigned)
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_children_garden(character));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "kitchen" && k == "children_garden" => {
                let event: Box<dyn Event> =
                    Box::new(events::make_move_to_children_garden("doggie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "kitchen" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_garden("kitie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "children_garden" => {
                // make picks
                world
                    .items()
                    .values()
                    .filter(|e| {
                        e.roles().contains(&"toy")
                            && e.state() == &ItemState::InScene("children_garden".into())
                    })
                    .for_each(|e| {
                        for character in ["doggie", "kitie"] {
                            let event: Box<dyn Event> = Box::new(events::make_pick(
                                "play".into(),
                                character,
                                e.name(),
                                true,
                            ));
                            res.push(event);
                        }
                    });

                // make move events
                if world
                    .items()
                    .values()
                    .filter(|e| e.roles().contains(&"toy"))
                    .all(|e| e.state() == &ItemState::Unassigned)
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_garden(character));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "children_garden" && k == "garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_garden("doggie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_garden("kitie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "garden" && k == "garden" => {
                if world.items().get("bad_dog").unwrap().state()
                    == &ItemState::InScene("garden".into())
                {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> = Box::new(events::make_find_bad_dog(character));
                        res.push(event);
                    }
                } else {
                    for character in ["doggie", "kitie"] {
                        let event: Box<dyn Event> =
                            Box::new(events::make_move_to_children_house(character));
                        res.push(event);
                    }
                }
            }
            (Some(d), Some(k)) if d == "garden" && k == "children_house" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_house("doggie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "garden" => {
                let event: Box<dyn Event> = Box::new(events::make_move_to_children_house("kitie"));
                res.push(event);
            }
            (Some(d), Some(k)) if d == "children_house" && k == "children_house" => {
                if !doggie.consumed_pie {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "doggie", "pie"));
                    res.push(event);
                }
                if !doggie.consumed_soup {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "doggie", "soup"));
                    res.push(event);
                }
                if !doggie.consumed_dumplings {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "doggie", "dumplings"));
                    res.push(event);
                }
                if !doggie.consumed_meat {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "doggie", "meat"));
                    res.push(event);
                }

                if !kitie.consumed_pie {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "kitie", "pie"));
                    res.push(event);
                }
                if !kitie.consumed_soup {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "kitie", "soup"));
                    res.push(event);
                }
                if !kitie.consumed_dumplings {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "kitie", "dumplings"));
                    res.push(event);
                }
                if !kitie.consumed_meat {
                    let event: Box<dyn Event> =
                        Box::new(events::make_eat_meal("eat".into(), "kitie", "meat"));
                    res.push(event);
                }
            }
            (Some(d), Some(k)) if d == "way_home" && k == "way_home" => {}
            _ => unreachable!(),
        }

        res
    }
}
