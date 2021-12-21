use super::characters;
use pabitell_lib::{conditions, data, events, updates, Character, Event, ItemState, Tagged, World};

use crate::translations::get_message;

fn doggie_and_kitie_in_same_scene(world: &dyn World) -> bool {
    conditions::same_scene(
        world,
        &vec!["doggie".to_string(), "kitie".to_string()],
        &vec![],
    )
    .unwrap()
}

// TODO talk event

/*
pub fn make_pick(pick_data: data::PickData, consume: bool) -> events::Pick {
    let mut event = events::Pick::new(pick_data);

    event.set_tags(vec!["pick".to_string()]);

    event.set_world_update(Some(Box::new(move |event, world| {
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
    })));

    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        let item = if let Some(item) = world.items().get(event.item()) {
            item
        } else {
            return false;
        };
        let ingredient_cond = if item.get_tags().contains(&"batch2".to_string()) {
            // All batch1 items done
            conditions::all_items_with_tags_in_state(
                world,
                &["batch1".to_string()],
                ItemState::Unassigned,
            )
        } else if item.get_tags().contains(&"batch3".to_string()) {
            // All batch1, batch2 items done
            conditions::all_items_with_tags_in_state(
                world,
                &["batch1".to_string(), "batch2".to_string()],
                ItemState::Unassigned,
            )
        } else if item.get_tags().contains(&"batch4".to_string()) {
            // All batch1, batch2, batch3 items done
            conditions::all_items_with_tags_in_state(
                world,
                &[
                    "batch1".to_string(),
                    "batch2".to_string(),
                    "batch3".to_string(),
                ],
                ItemState::Unassigned,
            )
        } else if item.get_tags().contains(&"batch5".to_string()) {
            // All batch1, batch2, batch3, batch4 items done
            conditions::all_items_with_tags_in_state(
                world,
                &[
                    "batch1".to_string(),
                    "batch2".to_string(),
                    "batch3".to_string(),
                    "batch4".to_string(),
                ],
                ItemState::Unassigned,
            )
        } else if item.get_tags().contains(&"batch6".to_string()) {
            // All batch1, batch2, batch3, batch4, batch5 items done
            conditions::all_items_with_tags_in_state(
                world,
                &[
                    "batch1".to_string(),
                    "batch2".to_string(),
                    "batch3".to_string(),
                    "batch4".to_string(),
                    "batch5".to_string(),
                ],
                ItemState::Unassigned,
            )
        } else {
            // not ingredient
            true
        };

        ingredient_cond
            && conditions::same_scene(
                world,
                &vec![event.character().to_string()],
                &vec![event.item().to_string()],
            )
            .unwrap()
            && doggie_and_kitie_in_same_scene(world)
    })));

    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        get_message(
            &format!(
                "{}-{}_{}_{}-action",
                world.name(),
                event.character(),
                event.name(),
                event.item()
            ),
            world.lang(),
            None,
        )
    })));

    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        get_message(
            &format!(
                "{}-{}_{}_{}-success",
                world.name(),
                event.character(),
                event.name(),
                event.item()
            ),
            world.lang(),
            None,
        )
    })));

    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        get_message(
            &format!(
                "{}-{}_{}_{}-fail",
                world.name(),
                event.character(),
                event.name(),
                event.item()
            ),
            world.lang(),
            None,
        )
    })));
    event
}
*/
