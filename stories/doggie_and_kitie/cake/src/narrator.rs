use pabitell_lib::{data, Event, Narrator, World};
use serde_json::Value;

use crate::events::{self, ProtocolEvent};

#[derive(Default, Debug)]
pub struct Cake;

impl Narrator for Cake {
    fn all_events(&self, world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut res: Vec<Box<dyn Event>> = vec![];

        // Move to kitchech
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move_to_kitchen(data::MoveData::new(
                character, "kitchen",
            ))));
        }

        // Pick Sand cake
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_pick(
                "pick",
                data::PickData::new(character, "sand_cake"),
                false,
            )));
        }

        // Give Sand cake
        for (from, to) in &[("doggie", "kitie"), ("kitie", "doggie")] {
            res.push(Box::new(events::make_give_sand_cake(data::GiveData::new(
                from,
                to,
                "sand_cake",
            ))));
        }

        // Pick ingredients
        for character in &["doggie", "kitie"] {
            for accepted_ingredient in world.items().values().filter_map(|e| {
                let tags = e.get_tags();
                if tags.contains(&"ingredient".to_string())
                    && tags.contains(&"accepted".to_string())
                {
                    Some(e.name())
                } else {
                    None
                }
            }) {
                res.push(Box::new(events::make_pick(
                    "pick_ingredient",
                    data::PickData::new(character, accepted_ingredient),
                    false,
                )));
            }
            for rejected_ingredient in world.items().values().filter_map(|e| {
                let tags = e.get_tags();
                if tags.contains(&"ingredient".to_string())
                    && tags.contains(&"rejected".to_string())
                {
                    Some(e.name())
                } else {
                    None
                }
            }) {
                res.push(Box::new(events::make_pick(
                    "pick_disliked_ingredient",
                    data::PickData::new(character, rejected_ingredient),
                    true,
                )));
            }
        }

        // Put ingredients to pot
        for character in &["doggie", "kitie"] {
            for ingredient in world.items().values().filter_map(|e| {
                let tags = e.get_tags();
                if tags.contains(&"ingredient".to_string())
                    && tags.contains(&"accepted".to_string())
                {
                    Some(e.name())
                } else {
                    None
                }
            }) {
                res.push(Box::new(events::make_use_item(
                    "add_ingredient",
                    data::UseItemData::new(character, ingredient),
                    true,
                )));
            }
        }

        // Move to children garden
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move_to_children_garden(
                data::MoveData::new(character, "children_garden"),
            )));
        }

        // Play with toys
        for character in &["doggie", "kitie"] {
            for toy in world.items().values().filter_map(|e| {
                if e.get_tags().contains(&"toy".to_string()) {
                    Some(e.name())
                } else {
                    None
                }
            }) {
                res.push(Box::new(events::make_pick(
                    "play",
                    data::PickData::new(character, toy),
                    true,
                )));
            }
        }

        // Move to garden
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move_to_garden(data::MoveData::new(
                character, "garden",
            ))));
        }

        // Find bad dog
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_find_bad_dog(data::PickData::new(
                character, "bad_dog",
            ))));
        }

        // Move to children house
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move_to_children_house(
                data::MoveData::new(character, "children_house"),
            )));
        }

        // Eat meal
        for character in &["doggie", "kitie"] {
            for meal in &["pie", "soup", "dumplings", "meat"] {
                res.push(Box::new(events::make_eat_meal(data::VoidData::new(
                    character,
                    Some(meal),
                ))));
            }
        }

        res
    }

    fn parse_event(&self, _world: &dyn World, value: Value) -> Option<Box<dyn Event>> {
        let event: Result<ProtocolEvent, serde_json::Error> = serde_json::from_value(value);

        match event {
            Ok(ProtocolEvent::MoveToKitchen(data)) => {
                Some(Box::new(events::make_move_to_kitchen(data)))
            }
            Ok(ProtocolEvent::MoveToChildrenGarden(data)) => {
                Some(Box::new(events::make_move_to_children_garden(data)))
            }
            Ok(ProtocolEvent::MoveToGarden(data)) => {
                Some(Box::new(events::make_move_to_garden(data)))
            }
            Ok(ProtocolEvent::MoveToChildrenHouse(data)) => {
                Some(Box::new(events::make_move_to_children_house(data)))
            }
            Ok(ProtocolEvent::PickIngredient(data)) => {
                Some(Box::new(events::make_pick("pick_ingredient", data, false)))
            }
            Ok(ProtocolEvent::Pick(data)) => Some(Box::new(events::make_pick("pick", data, false))),
            Ok(ProtocolEvent::GiveSandCake(data)) => {
                Some(Box::new(events::make_give_sand_cake(data)))
            }
            Ok(ProtocolEvent::Give(data)) => Some(Box::new(events::make_give(data))),
            Ok(ProtocolEvent::Play(data)) => Some(Box::new(events::make_pick("play", data, true))),
            Ok(ProtocolEvent::FindBadDog(data)) => Some(Box::new(events::make_find_bad_dog(data))),
            Ok(ProtocolEvent::Eat(data)) => Some(Box::new(events::make_eat_meal(data))),
            Ok(ProtocolEvent::PickDislikedIngredient(data)) => Some(Box::new(events::make_pick(
                "pick_disliked_ingredient",
                data,
                true,
            ))),
            Ok(ProtocolEvent::AddIngredient(data)) => Some(Box::new(events::make_use_item(
                "add_ingredient",
                data,
                true,
            ))),
            Err(_) => None,
        }
    }
}
