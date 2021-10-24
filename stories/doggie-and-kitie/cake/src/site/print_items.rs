use pabitell_lib::{data, World};
use serde_json::Value;
use std::rc::Rc;

use crate::translations::get_message;

use super::print::PrintItem;

fn remove_character_from_data(mut data: Value) -> Value {
    if let Value::Object(ref mut map) = data {
        map.remove("character");
    }
    data
}

pub fn make_print_items(world: &dyn World) -> Vec<PrintItem> {
    let mut res = vec![];

    // sand cake
    let item = world.items().get("sand_cake").unwrap();
    let data = remove_character_from_data(
        serde_json::to_value(data::PickData::new("pick", "", "sand_cake")).unwrap(),
    );
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world)))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to kitchen
    let scene = world.scenes().get("kitchen").unwrap();
    let data = remove_character_from_data(
        serde_json::to_value(data::MoveData::new("move_to_kitchen", "", "kitchen")).unwrap(),
    );
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world)))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // pick ingredients
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"ingredient".to_string()))
        .for_each(|e| {
            let event_name = if e.get_tags().contains(&"accepted".to_string()) {
                "pick_ingredient"
            } else {
                // rejected ingredient
                "pick_disliked_ingredient"
            }
            .to_string();

            let data = remove_character_from_data(
                serde_json::to_value(data::PickData::new(event_name, "", e.name())).unwrap(),
            );
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world)))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    // put into pot
    let mut data = remove_character_from_data(
        serde_json::to_value(data::UseItemData::new("add_ingredient", "", "")).unwrap(),
    );
    let data = {
        if let Value::Object(ref mut map) = data {
            map.remove("item");
        }
        data
    };
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(get_message(
                "doggie_and_kitie-cake-large_pot",
                &world.lang(),
                None,
            )))
            .img_url(Some("images/bit-pot.svg".to_owned())),
    );

    // move to children garden
    let scene = world.scenes().get("children_garden").unwrap();
    let data = remove_character_from_data(
        serde_json::to_value(data::MoveData::new(
            "move_to_children_garden",
            "",
            "children_garden",
        ))
        .unwrap(),
    );
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world)))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // play with toys
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"toy".to_string()))
        .for_each(|e| {
            let data = remove_character_from_data(
                serde_json::to_value(data::PickData::new("play", "", e.name())).unwrap(),
            );
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world)))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    // move to garden
    let scene = world.scenes().get("garden").unwrap();
    let data = remove_character_from_data(
        serde_json::to_value(data::MoveData::new("move_to_garden", "", "garden")).unwrap(),
    );
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world)))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // find bad dog
    let item = world.items().get("bad_dog").unwrap();
    let data = remove_character_from_data(
        serde_json::to_value(data::PickData::new("find", "", "bad_dog")).unwrap(),
    );
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world)))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to children house
    let scene = world.scenes().get("children_house").unwrap();
    let data = remove_character_from_data(
        serde_json::to_value(data::MoveData::new(
            "move_to_children_house",
            "",
            "children_house",
        ))
        .unwrap(),
    );
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world)))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // eat meals
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"meal".to_string()))
        .for_each(|e| {
            let data = remove_character_from_data(
                serde_json::to_value(data::VoidData::new("eat", "", Some(e.name()))).unwrap(),
            );
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world)))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    return res;
}
