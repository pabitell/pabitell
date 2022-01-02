use pabitell_lib::{data, webapp::print::PrintItem, World};
use serde_json::Value;
use std::rc::Rc;

use crate::translations::get_message;

pub fn make_print_items(world: Box<dyn World>) -> Vec<PrintItem> {
    let mut res = vec![];

    // move to walk
    let scene = world.scenes().get("walk").unwrap();
    let data = serde_json::to_value(data::MoveData::new("move_to_walk", "", "walk")).unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // find doll
    let item = world.items().get("doll").unwrap();
    let data = serde_json::to_value(data::PickData::new("pick", "", "doll")).unwrap();

    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // return home
    let scene = world.scenes().get("home").unwrap();
    let data = serde_json::to_value(data::MoveData::new("move_to_home", "", "home")).unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // move to doggie search
    let scene = world.scenes().get("doggie_search").unwrap();
    let data = serde_json::to_value(data::MoveData::new(
        "move_to_doggie_search",
        "",
        "doggie_search",
    ))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // move to doggie search
    let scene = world.scenes().get("kitie_search").unwrap();
    let data = serde_json::to_value(data::MoveData::new(
        "move_to_kitie_search",
        "",
        "kitie_search",
    ))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // doggie picks
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"doggie_pick".to_string()))
        .for_each(|e| {
            let data = serde_json::to_value(data::PickData::new("pick", "", e.name())).unwrap();
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world.as_ref())))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    // kitie picks
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"kitie_pick".to_string()))
        .for_each(|e| {
            let data = serde_json::to_value(data::PickData::new("pick", "", e.name())).unwrap();
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world.as_ref())))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    return res;
}
