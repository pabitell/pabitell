use anyhow::Result;
use pabitell_lib::{
    data::{GiveData, UseItemData},
    events::{Give, UseItem},
    webapp::items,
    ItemState, World,
};
use serde_json::Value;
use std::rc::Rc;

pub fn make_owned_items(world: &dyn World, character: &Option<String>) -> Rc<Vec<Rc<items::Item>>> {
    let res = if let Some(character) = character {
        let owned_state = ItemState::Owned(character.to_string());
        world
            .items()
            .values()
            .filter(|i| i.state() == &owned_state)
            .map(|i| {
                let data = GiveData::new(
                    format!("give_{}", i.name()),
                    character,
                    String::new(), // keep target character empty
                    i.name(),
                );
                Rc::new(items::Item {
                    code: i.name().to_string(),
                    short: i.short(world),
                    long: i.long(world),
                    image_url: format!("images/{}.svg", i.name()),
                    give_data: Some(Rc::new(serde_json::to_vec(&data).unwrap())),
                    use_data: None,
                    scan: i.name() != "sand_cake",
                    default: if i.name() == "sand_cake" {
                        items::DefaultAction::Give
                    } else {
                        items::DefaultAction::Scan
                    },
                })
            })
            .collect()
    } else {
        vec![]
    };
    Rc::new(res)
}
