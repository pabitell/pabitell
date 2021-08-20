use super::characters;
use pabitell_lib::{events, Character, Description, Event, Id, ItemState, Named, World};

use crate::translations::get_message;

pub fn make_pick(name: &str, character: &str, item: &str, consume: bool) -> events::Pick {
    events::Pick::new(
        name,
        character,
        item,
        consume,
        vec!["pick"],
        None,
        None,
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_give_sand_cake(from_character: &str, to_character: &str) -> events::Give {
    events::Give::new(
        "give",
        from_character,
        to_character,
        "sand_cake",
        true,
        vec!["give"],
        Some(Box::new(|event, world| {
            let character = world
                .characters_mut()
                .get_mut(event.to_character())
                .unwrap()
                .as_any_mut();

            if let Some(kitie) = character.downcast_mut::<characters::Kitie>() {
                kitie.sand_cake_last = true;
            }

            if let Some(doggie) = character.downcast_mut::<characters::Doggie>() {
                doggie.sand_cake_last = true;
            }
        })),
        None,
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_move_to_kitchen(character: &str) -> events::Move {
    events::Move::new(
        "move",
        character,
        vec!["playground"],
        "kitchen",
        vec!["move"],
        None,
        Some(Box::new(|_event, world| {
            world.items().get("sand_cake").unwrap().state() == &ItemState::Unassigned
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_disliked_pick(name: &str, character: &str, item: &str) -> events::Void {
    events::Void::new(
        name,
        character,
        Some(item),
        vec!["kitchen".into()],
        None,
        None,
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_move_to_children_garden(character: &str) -> events::Move {
    events::Move::new(
        "move",
        character,
        vec!["kitchen"],
        "children_garden",
        vec!["move"],
        None,
        Some(Box::new(|_event, world| {
            // Everything is in the cake
            world
                .items()
                .values()
                .filter(|e| e.roles().contains(&"accepted"))
                .all(|e| e.state() == &ItemState::Unassigned)
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_use_item(name: &str, character: &str, item: &str, consume: bool) -> events::UseItem {
    events::UseItem::new(
        name,
        character,
        item.to_string(),
        consume,
        vec!["use_item"],
        None,
        Some(Box::new(|event, world| {
            // all characters in the same scene
            let scene = world.characters().get(event.character()).unwrap().scene();
            world.characters().values().all(|e| e.scene() == scene)
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_move_to_garden(character: &str) -> events::Move {
    events::Move::new(
        "move",
        character,
        vec!["children_garden"],
        "garden",
        vec!["move"],
        None,
        Some(Box::new(|_event, world| {
            // Everything is in the cake
            world
                .items()
                .values()
                .filter(|e| e.roles().contains(&"toy"))
                .all(|e| e.state() == &ItemState::Unassigned)
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_find_bad_dog(character: &str) -> events::Pick {
    events::Pick::new(
        "find",
        character,
        "bad_dog",
        true,
        vec![],
        None,
        Some(Box::new(|_, world| {
            world
                .characters()
                .values()
                .all(|e| e.scene().clone() == Some("garden".to_string()))
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_move_to_children_house(character: &str) -> events::Move {
    events::Move::new(
        "move",
        character,
        vec!["garden"],
        "children_house",
        vec!["move"],
        None,
        Some(Box::new(|_event, world| {
            // Found bad dog
            world.items().get("bad_dog").unwrap().state() == &ItemState::Unassigned
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}

pub fn make_eat_meal(name: &str, character: &str, item: &str) -> events::Void {
    events::Void::new(
        name,
        character,
        Some(item),
        vec!["children_house".into()],
        Some(Box::new(|event, world| {
            // mark consumed
            let character = world
                .characters_mut()
                .get_mut(event.character())
                .unwrap()
                .as_any_mut();

            if let Some(kitie) = character.downcast_mut::<characters::Kitie>() {
                match event.item() {
                    Some(i) if i == "meat" => kitie.consumed_meat = true,
                    Some(i) if i == "dumplings" => kitie.consumed_dumplings = true,
                    Some(i) if i == "soup" => kitie.consumed_soup = true,
                    Some(i) if i == "pie" => kitie.consumed_pie = true,
                    _ => unreachable!(),
                }
            }

            if let Some(doggie) = character.downcast_mut::<characters::Doggie>() {
                match event.item() {
                    Some(i) if i == "meat" => doggie.consumed_meat = true,
                    Some(i) if i == "dumplings" => doggie.consumed_dumplings = true,
                    Some(i) if i == "soup" => doggie.consumed_soup = true,
                    Some(i) if i == "pie" => doggie.consumed_pie = true,
                    _ => unreachable!(),
                }
            }

            // test if doggie and kitie are ready to go
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

            // move to final scene
            if kitie.full() && doggie.full() {
                let doggie = world
                    .characters_mut()
                    .get_mut("doggie")
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<characters::Doggie>()
                    .unwrap();
                doggie.set_scene(Some("way_home".into()));
                let kitie = world
                    .characters_mut()
                    .get_mut("kitie")
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<characters::Kitie>()
                    .unwrap();
                kitie.set_scene(Some("way_home".into()));
            }
        })),
        Some(Box::new(|event, world| {
            // item is meal
            if let Some(item) = event.item() {
                world.items().get(item).unwrap().roles().contains(&"meal")
            } else {
                false
            }
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}
