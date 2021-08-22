use super::characters;
use pabitell_lib::{conditions, events, updates, Character, Event, ItemState};

use crate::translations::get_message;

pub fn make_pick(name: &str, character: &str, item: &str, consume: bool) -> events::Pick {
    events::Pick::new(
        name,
        character,
        item,
        vec!["pick"],
        Some(Box::new(move |event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();

            updates::assign_item(
                world,
                event.item().to_string(),
                if consume {
                    ItemState::Unassigned
                } else {
                    ItemState::Owned(event.character().to_string())
                },
            )
            .unwrap();
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
            conditions::same_scene(
                world,
                &vec![event.character().to_string()],
                &vec![event.item().to_string()],
            )
            .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
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
        vec!["give"],
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Give>().unwrap();

            updates::assign_item(world, event.item().to_string(), ItemState::Unassigned).unwrap();

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
            let event = event.downcast_ref::<events::Give>().unwrap();
            conditions::can_give(
                world,
                event.from_character().to_string(),
                event.to_character().to_string(),
                event.item().to_string(),
            )
            .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Give>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Give>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Give>().unwrap();
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
        "kitchen",
        vec!["move"],
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            updates::move_character(
                world,
                event.character().to_string(),
                Some(event.scene().to_string()),
            )
            .unwrap();
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            world.items().get("sand_cake").unwrap().state() == &ItemState::Unassigned
                && conditions::in_scenes(
                    world,
                    event.character().to_string(),
                    &vec!["playground".into()],
                )
                .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
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
        None,
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
            let character = world.characters().get(event.character()).unwrap();
            character.scene() == &Some("kitchen".to_string())
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
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
        "children_garden",
        vec!["move"],
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            updates::move_character(
                world,
                event.character().to_string(),
                Some(event.scene().to_string()),
            )
            .unwrap();
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            // Everything is in the cake
            world
                .items()
                .values()
                .filter(|e| e.roles().contains(&"accepted"))
                .all(|e| e.state() == &ItemState::Unassigned)
                && conditions::in_scenes(
                    world,
                    event.character().to_string(),
                    &vec!["kitchen".into()],
                )
                .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
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
        vec!["use_item"],
        Some(Box::new(move |event, world| {
            let event = event.downcast_ref::<events::UseItem>().unwrap();
            if consume {
                updates::assign_item(world, event.item().to_string(), ItemState::Unassigned)
                    .unwrap();
            }
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::UseItem>().unwrap();
            conditions::same_scene(
                world,
                &world
                    .characters()
                    .values()
                    .map(|e| e.name().to_string())
                    .collect::<Vec<_>>(),
                &vec![],
            )
            .unwrap()
                && conditions::has_item(
                    world,
                    event.character().to_string(),
                    event.item().to_string(),
                )
                .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::UseItem>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::UseItem>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::UseItem>().unwrap();
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
        "garden",
        vec!["move"],
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            updates::move_character(
                world,
                event.character().to_string(),
                Some(event.scene().to_string()),
            )
            .unwrap();
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            // Everything is in the cake
            world
                .items()
                .values()
                .filter(|e| e.roles().contains(&"toy"))
                .all(|e| e.state() == &ItemState::Unassigned)
                && conditions::in_scenes(
                    world,
                    event.character().to_string(),
                    &vec!["children_garden".into()],
                )
                .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
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
        vec![],
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();

            updates::assign_item(world, event.item().to_string(), ItemState::Unassigned).unwrap();
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
            conditions::same_scene(
                world,
                &world
                    .characters()
                    .values()
                    .map(|e| e.name().to_string())
                    .collect::<Vec<_>>(),
                &vec![event.item().to_string()],
            )
            .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Pick>().unwrap();
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
        "children_house",
        vec!["move"],
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            updates::move_character(
                world,
                event.character().to_string(),
                Some(event.scene().to_string()),
            )
            .unwrap();
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            // Found bad dog
            world.items().get("bad_dog").unwrap().state() == &ItemState::Unassigned
                && conditions::in_scenes(
                    world,
                    event.character().to_string(),
                    &vec!["garden".into()],
                )
                .unwrap()
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Move>().unwrap();
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
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
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
            let event = event.downcast_ref::<events::Void>().unwrap();

            let character = world.characters().get(event.character()).unwrap();
            if character.scene() != &Some("children_house".into()) {
                return false;
            }
            // item is meal
            if let Some(item) = event.item() {
                world.items().get(item).unwrap().roles().contains(&"meal")
            } else {
                false
            }
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
            get_message(
                &format!("{}-{}-action", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
            get_message(
                &format!("{}-{}-success", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
        Some(Box::new(|event, world| {
            let event = event.downcast_ref::<events::Void>().unwrap();
            get_message(
                &format!("{}-{}-fail", world.name(), event.translation_base(),),
                world.lang(),
                None,
            )
        })),
    )
}
