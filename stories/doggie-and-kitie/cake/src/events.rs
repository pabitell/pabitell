use super::characters;
use pabitell_lib::{events, Character, Description, Event, Id, ItemState, Named, World};
use std::any::Any;
use uuid::Uuid;

use crate::translations::get_message;

pub fn make_pick(
    name: String,
    character: &'static str,
    item: &'static str,
    consume: bool,
) -> events::Pick {
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

pub fn make_give_sand_cake(
    from_character: &'static str,
    to_character: &'static str,
) -> events::Give {
    events::Give::new(
        "give".into(),
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

pub fn make_move_to_kitchen(character: &'static str) -> events::Move {
    events::Move::new(
        "move".into(),
        character,
        vec!["playground"],
        "kitchen",
        vec!["move"],
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

pub fn make_disliked_pick(
    name: String,
    character: &'static str,
    item: &'static str,
) -> events::Void {
    events::Void::new(
        name,
        character,
        Some(item),
        Some(vec!["kitchen"]),
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

pub fn make_move_to_children_garden(character: &'static str) -> events::Move {
    events::Move::new(
        "move".into(),
        character,
        vec!["kitchen"],
        "children_garden",
        vec!["move"],
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

pub fn make_use_item(
    name: String,
    character: &'static str,
    item: &'static str,
    consume: bool,
) -> events::UseItem {
    events::UseItem::new(
        name,
        character,
        item,
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

pub fn make_move_to_garden(character: &'static str) -> events::Move {
    events::Move::new(
        "move".into(),
        character,
        vec!["children_garden"],
        "garden",
        vec!["move"],
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

pub fn make_find_bad_dog(character: &'static str) -> events::Pick {
    events::Pick::new(
        "find".into(),
        character,
        "bad_dog",
        true,
        vec![],
        None,
        Some(Box::new(|_, world| {
            world
                .characters()
                .values()
                .all(|e| e.scene() == Some("garden"))
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

pub fn make_move_to_children_house(character: &'static str) -> events::Move {
    events::Move::new(
        "move".into(),
        character,
        vec!["garden"],
        "children_house",
        vec!["move"],
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

pub fn make_eat_meal(name: String, character: &'static str, item: &'static str) -> events::Void {
    events::Void::new(
        name,
        character,
        Some(item),
        Some(vec!["children_house"]),
        Some(Box::new(|event, world| {
            // mark consumed
            let character = world
                .characters_mut()
                .get_mut(event.character())
                .unwrap()
                .as_any_mut();

            if let Some(kitie) = character.downcast_mut::<characters::Kitie>() {
                match event.item() {
                    Some("meat") => kitie.consumed_meat = true,
                    Some("dumplings") => kitie.consumed_dumplings = true,
                    Some("soup") => kitie.consumed_soup = true,
                    Some("pie") => kitie.consumed_pie = true,
                    _ => unreachable!(),
                }
            }

            if let Some(doggie) = character.downcast_mut::<characters::Doggie>() {
                match event.item() {
                    Some("meat") => doggie.consumed_meat = true,
                    Some("dumplings") => doggie.consumed_dumplings = true,
                    Some("soup") => doggie.consumed_soup = true,
                    Some("pie") => doggie.consumed_pie = true,
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
                doggie.set_scene(Some("way_home"));
                let kitie = world
                    .characters_mut()
                    .get_mut("kitie")
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<characters::Kitie>()
                    .unwrap();
                kitie.set_scene(Some("way_home"));
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
