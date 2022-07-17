use pabitell_lib::{data, webapp::print::PrintItem, World};
use serde_json::Value;
use std::rc::Rc;

use crate::events::ProtocolEvent;

pub fn make_print_items(world: Box<dyn World>) -> Vec<PrintItem> {
    let mut res = vec![];

    // sand cake
    let item = world.items().get("sand_cake").unwrap();
    let data =
        serde_json::to_value(ProtocolEvent::Pick(data::PickData::new("", "sand_cake"))).unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to kitchen
    let scene = world.scenes().get("kitchen").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToKitchen(data::MoveData::new(
        "", "kitchen",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // pick ingredients
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"ingredient".to_string()))
        .for_each(|e| {
            let data = if e.get_tags().contains(&"accepted".to_string()) {
                serde_json::to_value(ProtocolEvent::PickIngredient(data::PickData::new(
                    "",
                    e.name(),
                )))
                .unwrap()
            } else {
                // rejected ingredient
                serde_json::to_value(ProtocolEvent::PickDislikedIngredient(data::PickData::new(
                    "",
                    e.name(),
                )))
                .unwrap()
            }
            .to_string();

            res.push(
                PrintItem::new(Rc::new(data.as_bytes().to_vec()))
                    .title(Some(e.short(world.as_ref())))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    // put into pot
    let mut data =
        serde_json::to_value(ProtocolEvent::AddIngredient(data::UseItemData::new("", ""))).unwrap();
    let data = {
        if let Value::Object(ref mut map) = data {
            map.remove("item");
        }
        data
    };
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(
                world.get_message("doggie_and_kitie-cake-large_pot", None),
            ))
            .img_url(Some("images/big-pot.svg".to_owned())),
    );

    // move to children garden
    let scene = world.scenes().get("children_garden").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToChildrenGarden(data::MoveData::new(
        "",
        "children_garden",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // play with toys
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"toy".to_string()))
        .for_each(|e| {
            let data = serde_json::to_value(ProtocolEvent::Play(data::PickData::new("", e.name())))
                .unwrap();
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world.as_ref())))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    // move to garden
    let scene = world.scenes().get("garden").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToGarden(data::MoveData::new(
        "", "garden",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // find bad dog
    let item = world.items().get("bad_dog").unwrap();
    let data = serde_json::to_value(ProtocolEvent::FindBadDog(data::PickData::new(
        "", "bad_dog",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to children house
    let scene = world.scenes().get("children_house").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToChildrenHouse(data::MoveData::new(
        "",
        "children_house",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // eat meals
    world
        .items()
        .values()
        .filter(|e| e.get_tags().contains(&"meal".to_string()))
        .for_each(|e| {
            let data =
                serde_json::to_value(ProtocolEvent::Eat(data::VoidData::new("", Some(e.name()))))
                    .unwrap();
            res.push(
                PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
                    .title(Some(e.short(world.as_ref())))
                    .img_url(Some(format!("images/{}.svg", e.name()))),
            );
        });

    res
}
