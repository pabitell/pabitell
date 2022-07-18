use pabitell_lib::{conditions, data, events, updates, Event, ItemState, Tagged, World};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ProtocolEvent {
    TalkInHome(data::TalkData),
    MoveToWalk(data::MoveData),
    TalkOnWalk(data::TalkData),
    FindDoll(data::UseItemData),
    MoveToHome(data::MoveData),
    MoveToDoggieSearch(data::MoveData),
    MoveToKitieSearch(data::MoveData),
    Pick(data::PickData),
    LayDown(data::UseItemData),
}

fn doggie_and_kitie_in_same_scene(world: &dyn World) -> bool {
    conditions::same_scene(world, &["doggie".to_string(), "kitie".to_string()], &[]).unwrap()
}

pub fn make_talk(name: &str, data: data::TalkData) -> events::Talk {
    let mut event = events::Talk::new(name, data);

    event.set_tags(vec!["talk".to_string()]);
    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Talk>().unwrap();
        if !conditions::in_scenes(
            world,
            event.character().to_string(),
            &[event.scene().to_owned()],
        )
        .unwrap()
        {
            return false;
        }
        if !conditions::scene_dialog(world, event.scene(), event.dialog()).unwrap() {
            return false;
        }
        true
    })));

    event.set_world_update(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Talk>().unwrap();
        updates::next_scene_dialog(world, event.scene().to_owned()).unwrap();
    })));

    event.set_make_action_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Talk>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_{}_says-{}-action",
                world.name(),
                event.scene(),
                event.character(),
                event.dialog(),
            ),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Talk>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_{}_says-{}-success",
                world.name(),
                event.scene(),
                event.character(),
                event.dialog(),
            ),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Talk>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_{}_says-{}-fail",
                world.name(),
                event.scene(),
                event.character(),
                event.dialog(),
            ),
            None,
        )
    })));
    event.set_tags(vec!["no_read".to_string()]);

    event
}

pub fn make_move(
    name: &str,
    data: data::MoveData,
    from_scene: &str,
    from_dialog: Option<usize>,
    items_state: Option<(Vec<String>, ItemState)>,
    increase_dialog: bool,
) -> events::Move {
    let mut event = events::Move::new(name, data);
    let from_scene = from_scene.to_owned();

    event.set_tags(vec!["move".to_string()]);

    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        if !conditions::in_scenes(world, event.character().to_string(), &[from_scene.clone()])
            .unwrap()
        {
            return false;
        }
        if let Some(from_dialog) = from_dialog {
            if !conditions::scene_dialog(world, &from_scene, from_dialog).unwrap() {
                return false;
            }
        }
        if let Some((tags, state)) = items_state.as_ref() {
            if !conditions::all_items_with_tags_in_state(world, tags, state.clone()) {
                return false;
            }
        }
        true
    })));

    event.set_world_update(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        updates::move_character(
            world,
            event.character().to_string(),
            Some(event.scene().to_string()),
        )
        .unwrap();
        if increase_dialog {
            updates::next_scene_dialog(world, event.scene().to_string()).unwrap();
        }
    })));

    event.set_make_action_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_move_to_{}-action",
                world.name(),
                event.character(),
                event.scene(),
            ),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_move_to_{}-success",
                world.name(),
                event.character(),
                event.scene(),
            ),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_move_to_{}-fail",
                world.name(),
                event.character(),
                event.scene(),
            ),
            None,
        )
    })));

    event
}

pub fn make_find_doll(data: data::UseItemData) -> events::UseItem {
    let mut event = events::UseItem::new("find_doll", data);

    event.set_tags(vec!["find".to_string()]);

    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        if !&["doggie", "kitie"].contains(&event.character()) {
            return false;
        }
        if !conditions::same_scene(
            world,
            &["doggie".to_string(), "kitie".to_string()],
            &["doll".to_string()],
        )
        .unwrap()
        {
            return false;
        }
        if !conditions::scene_dialog(world, "walk", 4).unwrap() {
            return false;
        }
        true
    })));

    event.set_world_update(Some(Box::new(move |_, world| {
        updates::assign_item(world, "doll".to_owned(), ItemState::Unassigned).unwrap();
        updates::next_scene_dialog(world, "walk".to_owned()).unwrap();
    })));

    event.set_make_action_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        world.get_message(
            &format!("{}-{}_find_doll-action", world.name(), event.character(),),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        world.get_message(
            &format!("{}-{}_find_doll-success", world.name(), event.character(),),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        world.get_message(
            &format!("{}-{}_find_doll-fail", world.name(), event.character(),),
            None,
        )
    })));

    event
}

pub fn make_pick(name: &str, pick_data: data::PickData, scene: Option<String>) -> events::Pick {
    let mut event = events::Pick::new(name, pick_data);

    event.set_tags(vec!["pick".to_string()]);

    event.set_world_update(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        updates::assign_item(
            world,
            event.item().to_string(),
            ItemState::Owned(event.character().to_string()),
        )
        .unwrap();
    })));

    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        conditions::same_scene(
            world,
            &[event.character().to_string()],
            &[event.item().to_string()],
        )
        .unwrap_or(false)
            && scene
                .as_ref()
                .map(|s| {
                    conditions::in_scenes(world, event.character().to_owned(), &[s.to_owned()])
                        .unwrap()
                })
                .unwrap_or(true)
    })));

    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_{}_{}-action",
                world.name(),
                event.character(),
                event.name(),
                event.item()
            ),
            None,
        )
    })));

    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_{}_{}-success",
                world.name(),
                event.character(),
                event.name(),
                event.item()
            ),
            None,
        )
    })));

    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::Pick>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_{}_{}-fail",
                world.name(),
                event.character(),
                event.name(),
                event.item()
            ),
            None,
        )
    })));
    event
}

pub fn make_lay_down(name: &str, use_item_data: data::UseItemData) -> events::UseItem {
    let mut event = events::UseItem::new(name, use_item_data);
    event.set_tags(vec!["lay_down".to_string()]);

    event.set_world_update(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        updates::assign_item(
            world,
            event.item().to_string(),
            ItemState::InScene("home".to_string()),
        )
        .unwrap();
        // all done => update dialog
        if world
            .items()
            .values()
            .filter(|i| {
                i.get_tags().contains(&"doggie_pick".to_owned())
                    || i.get_tags().contains(&"kitie_pick".to_owned())
            })
            .all(|i| i.state() == &ItemState::InScene("home".to_string()))
        {
            updates::next_scene_dialog(world, "home".to_string()).unwrap();
        }
    })));

    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        doggie_and_kitie_in_same_scene(world)
            && conditions::has_item(
                world,
                event.character().to_string(),
                event.item().to_string(),
            )
            .unwrap()
            && conditions::in_scenes(world, event.character().to_string(), &["home".to_string()])
                .unwrap()
            && conditions::scene_dialog(world, "home", 17).unwrap()
    })));
    event.set_make_action_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_lay_down_{}-action",
                world.name(),
                event.character(),
                event.item(),
            ),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_lay_down_{}-success",
                world.name(),
                event.character(),
                event.item(),
            ),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(|event, world| {
        let event = event.downcast_ref::<events::UseItem>().unwrap();
        world.get_message(
            &format!(
                "{}-{}_lay_down_{}-fail",
                world.name(),
                event.character(),
                event.item(),
            ),
            None,
        )
    })));
    event
}
