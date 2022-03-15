use super::characters;
use pabitell_lib::{conditions, data, events, updates, Character, Event, ItemState, Tagged, World};
use serde::{Deserialize, Serialize};

use crate::translations::get_message;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ProtocolEvent {
    MoveToKitchen(data::MoveData),
    MoveToChildrenGarden(data::MoveData),
    MoveToGarden(data::MoveData),
    MoveToChildrenHouse(data::MoveData),
    PickIngredient(data::PickData),
    GiveSandCake(data::GiveData),
    Pick(data::PickData),
    Give(data::GiveData),
    Play(data::PickData),
    FindBadDog(data::PickData),
    Eat(data::VoidData),
    PickDislikedIngredient(data::PickData),
    AddIngredient(data::UseItemData),
}

fn doggie_and_kitie_in_same_scene(world: &dyn World) -> bool {
    conditions::same_scene(
        world,
        &vec!["doggie".to_string(), "kitie".to_string()],
        &vec![],
    )
    .unwrap()
}

pub fn make_pick(name: &str, pick_data: data::PickData, consume: bool) -> events::Pick {
    let mut event = events::Pick::new(name, pick_data);

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

pub fn make_give_sand_cake(give_data: data::GiveData) -> events::Give {
    let mut event = events::Give::new("give_sand_cake", give_data);
    event.set_tags(vec!["give".to_string()]);

    event.set_world_update(Some(Box::new(|event, world| {
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
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        conditions::can_give(
            world,
            event.from_character().to_string(),
            event.to_character().to_string(),
            event.item().to_string(),
        )
        .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        get_message(
            &format!(
                "{}-{}_give_{}_to_{}-action",
                world.name(),
                event.from_character(),
                event.item(),
                event.to_character()
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        get_message(
            &format!(
                "{}-{}_give_{}_to_{}-success",
                world.name(),
                event.from_character(),
                event.item(),
                event.to_character()
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        get_message(
            &format!(
                "{}-{}_give_{}_to_{}-fail",
                world.name(),
                event.from_character(),
                event.item(),
                event.to_character()
            ),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_give(give_data: data::GiveData) -> events::Give {
    let mut event = events::Give::new("give", give_data);
    event.set_tags(vec!["give".to_string()]);

    event.set_world_update(Some(Box::new(|event, world| {
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
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        conditions::can_give(
            world,
            event.from_character().to_string(),
            event.to_character().to_string(),
            event.item().to_string(),
        )
        .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        get_message(
            &format!(
                "{}-{}_give_{}_to_{}-action",
                world.name(),
                event.from_character(),
                event.item(),
                event.to_character()
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        get_message(
            &format!(
                "{}-{}_give_{}_to_{}-success",
                world.name(),
                event.from_character(),
                event.item(),
                event.to_character()
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Give>().unwrap();
        get_message(
            &format!(
                "{}-{}_give_{}_to_{}-fail",
                world.name(),
                event.from_character(),
                event.item(),
                event.to_character()
            ),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_move_to_kitchen(move_data: data::MoveData) -> events::Move {
    let mut event = events::Move::new("move_to_kitchen", move_data);
    event.set_tags(vec!["move".to_string()]);
    event.set_world_update(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        updates::move_character(
            world,
            event.character().to_string(),
            Some(event.scene().to_string()),
        )
        .unwrap();
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        world.items().get("sand_cake").unwrap().state() == &ItemState::Unassigned
            && conditions::in_scenes(
                world,
                event.character().to_string(),
                &vec!["playground".into()],
            )
            .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_kitchen-action",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_kitchen-success",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_kitchen-fail",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_move_to_children_garden(move_data: data::MoveData) -> events::Move {
    let mut event = events::Move::new("move_to_children_garden", move_data);
    event.set_tags(vec!["move".to_string()]);
    event.set_world_update(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        updates::move_character(
            world,
            event.character().to_string(),
            Some(event.scene().to_string()),
        )
        .unwrap();
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        // Everything is in the cake
        world
            .items()
            .values()
            .filter(|e| e.get_tags().contains(&"ingredient".to_string()))
            .all(|e| e.state() == &ItemState::Unassigned)
            && conditions::in_scenes(
                world,
                event.character().to_string(),
                &vec!["kitchen".into()],
            )
            .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_children_garden-action",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_children_garden-success",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_children_garden-fail",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_use_item(
    name: &str,
    use_item_data: data::UseItemData,
    consume: bool,
) -> events::UseItem {
    let mut event = events::UseItem::new(name, use_item_data);
    event.set_tags(vec!["use_item".to_string()]);
    event.set_world_update(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        if consume {
            updates::assign_item(world, event.item().to_string(), ItemState::Unassigned).unwrap();
        }
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        doggie_and_kitie_in_same_scene(world)
            && conditions::has_item(
                world,
                event.character().to_string(),
                event.item().to_string(),
            )
            .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        get_message(
            &format!(
                "{}-{}_use_{}-action",
                world.name(),
                event.character(),
                event.item(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        get_message(
            &format!(
                "{}-{}_use_{}-success",
                world.name(),
                event.character(),
                event.item(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        get_message(
            &format!(
                "{}-{}_use_{}-fail",
                world.name(),
                event.character(),
                event.item(),
            ),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_move_to_garden(move_data: data::MoveData) -> events::Move {
    let mut event = events::Move::new("move_to_garden", move_data);
    event.set_tags(vec!["move".to_string()]);

    event.set_world_update(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        updates::move_character(
            world,
            event.character().to_string(),
            Some(event.scene().to_string()),
        )
        .unwrap();
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        // Everything is in the cake
        world
            .items()
            .values()
            .filter(|e| e.get_tags().contains(&"toy".to_string()))
            .all(|e| e.state() == &ItemState::Unassigned)
            && conditions::in_scenes(
                world,
                event.character().to_string(),
                &vec!["children_garden".into()],
            )
            .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_garden-action",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_garden-success",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!("{}-{}_move_to_garden-fail", world.name(), event.character(),),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_find_bad_dog(pick_data: data::PickData) -> events::Pick {
    let mut event = events::Pick::new("find_bad_dog", pick_data);
    event.set_tags(vec!["find".to_string()]);

    event.set_world_update(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();

        updates::assign_item(world, event.item().to_string(), ItemState::Unassigned).unwrap();
    })));
    event.set_condition(Some(Box::new(|event, world| {
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
            && doggie_and_kitie_in_same_scene(world)
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        get_message(
            &format!("{}-{}_find_bad_dog-action", world.name(), event.character(),),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        get_message(
            &format!(
                "{}-{}_find_bad_dog-success",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        get_message(
            &format!("{}-{}_find_bad_dog-fail", world.name(), event.character(),),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_move_to_children_house(move_data: data::MoveData) -> events::Move {
    let mut event = events::Move::new("move_to_children_house", move_data);
    event.set_tags(vec!["move".to_string()]);
    event.set_world_update(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        updates::move_character(
            world,
            event.character().to_string(),
            Some(event.scene().to_string()),
        )
        .unwrap();
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        // Found bad dog
        world.items().get("bad_dog").unwrap().state() == &ItemState::Unassigned
            && conditions::in_scenes(world, event.character().to_string(), &vec!["garden".into()])
                .unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_children_house-action",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_children_house-success",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_children_house-fail",
                world.name(),
                event.character(),
            ),
            world.lang(),
            None,
        )
    })));
    event
}

pub fn make_eat_meal(void_data: data::VoidData) -> events::Void {
    let mut event = events::Void::new("eat", void_data);
    event.set_tags(vec!["eat".to_string()]);

    event.set_world_update(Some(Box::new(|event, world| {
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
    })));
    event.set_condition(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        if !doggie_and_kitie_in_same_scene(world) {
            return false;
        }

        let character = world.characters().get(event.character()).unwrap();
        if character.scene() != &Some("children_house".into()) {
            return false;
        }
        // item is meal
        if let Some(item) = event.item() {
            world
                .items()
                .get(item)
                .unwrap()
                .get_tags()
                .contains(&"meal".to_string())
        } else {
            false
        }
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        get_message(
            &format!(
                "{}-{}_eat_{}-action",
                world.name(),
                event.character(),
                event.item().clone().unwrap_or_else(String::new)
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        get_message(
            &format!(
                "{}-{}_eat_{}-success",
                world.name(),
                event.character(),
                event.item().clone().unwrap_or_else(String::new)
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        get_message(
            &format!(
                "{}-{}_eat_{}-fail",
                world.name(),
                event.character(),
                event.item().clone().unwrap_or_else(String::new)
            ),
            world.lang(),
            None,
        )
    })));
    event
}
