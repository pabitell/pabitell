use pabitell_lib::{data, webapp::print::PrintItem, World};
use std::rc::Rc;

use crate::events::ProtocolEvent;

pub fn make_print_items(world: Box<dyn World>) -> Vec<PrintItem> {
    let mut res = vec![];

    // move to walk
    let scene = world.scenes().get("walk").unwrap();
    let data =
        serde_json::to_value(ProtocolEvent::MoveToWalk(data::MoveData::new("", "walk"))).unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // find doll
    let item = world.items().get("doll").unwrap();
    let data =
        serde_json::to_value(ProtocolEvent::FindDoll(data::UseItemData::new("", "doll"))).unwrap();

    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // return home
    let scene = world.scenes().get("home").unwrap();
    let data =
        serde_json::to_value(ProtocolEvent::MoveToHome(data::MoveData::new("", "home"))).unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // move to doggie search
    let scene = world.scenes().get("doggie_search").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToDoggieSearch(data::MoveData::new(
        "",
        "doggie_search",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // move to doggie search
    let scene = world.scenes().get("kitie_search").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToKitieSearch(data::MoveData::new(
        "",
        "kitie_search",
    )))
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
            let data = serde_json::to_value(ProtocolEvent::Pick(data::PickData::new("", e.name())))
                .unwrap();
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
            let data = serde_json::to_value(ProtocolEvent::Pick(data::PickData::new("", e.name())))
                .unwrap();
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world.as_ref())))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    res
}
