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

pub fn make_talk(
    data: data::VoidData,
    scene: &str,
    characters: &[&str],
    dialog: usize,
) -> events::Void {
    let mut event = events::Void::new(data);
    let characters: Vec<String> = characters.iter().map(|e| e.to_string()).collect();
    let scene = scene.to_owned();

    event.set_tags(vec!["talk".to_string()]);
    let scene_cloned = scene.clone();
    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        if !characters.contains(&event.character().to_string()) {
            return false;
        }
        if !conditions::same_scene(
            world,
            &characters
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            &[],
        )
        .unwrap()
        {
            return false;
        }
        if !conditions::in_scenes(
            world,
            event.character().to_string(),
            &[scene_cloned.clone()],
        )
        .unwrap()
        {
            return false;
        }
        if !conditions::scene_dialog(world, &scene_cloned, dialog).unwrap() {
            return false;
        }
        true
    })));

    let scene_cloned = scene.clone();
    event.set_world_update(Some(Box::new(move |_, world| {
        updates::next_scene_dialog(world, scene_cloned.clone()).unwrap();
    })));

    let scene_cloned = scene.clone();
    event.set_make_action_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        get_message(
            &format!(
                "{}-{}_{}_says-{}-action",
                world.name(),
                scene_cloned,
                event.character(),
                dialog,
            ),
            world.lang(),
            None,
        )
    })));
    let scene_cloned = scene.clone();
    event.set_make_success_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        get_message(
            &format!(
                "{}-{}_{}_says-{}-success",
                world.name(),
                scene_cloned,
                event.character(),
                dialog,
            ),
            world.lang(),
            None,
        )
    })));
    let scene_cloned = scene.clone();
    event.set_make_fail_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Void>().unwrap();
        get_message(
            &format!(
                "{}-{}_{}_says-{}-fail",
                world.name(),
                scene_cloned,
                event.character(),
                dialog,
            ),
            world.lang(),
            None,
        )
    })));

    event
}

pub fn make_move(
    data: data::MoveData,
    character: &str,
    from_scene: &str,
    from_dialog: usize,
) -> events::Move {
    let mut event = events::Move::new(data);
    let character = character.to_owned();
    let from_scene = from_scene.to_owned();

    event.set_tags(vec!["move".to_string()]);

    event.set_condition(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        if event.character() != character {
            return false;
        }
        if !conditions::in_scenes(world, event.character().to_string(), &[from_scene.clone()])
            .unwrap()
        {
            return false;
        }
        if !conditions::scene_dialog(world, &from_scene, from_dialog).unwrap() {
            return false;
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
    })));

    event.set_make_action_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_{}-action",
                world.name(),
                event.character(),
                event.scene(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_success_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_{}-success",
                world.name(),
                event.character(),
                event.scene(),
            ),
            world.lang(),
            None,
        )
    })));
    event.set_make_fail_text(Some(Box::new(move |event, world| {
        let event = event.downcast_ref::<events::Move>().unwrap();
        get_message(
            &format!(
                "{}-{}_move_to_{}-fail",
                world.name(),
                event.character(),
                event.scene(),
            ),
            world.lang(),
            None,
        )
    })));

    event
}
