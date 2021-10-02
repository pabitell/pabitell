use anyhow::Result;
use pabitell_lib::{data::GiveData, events::Give, ItemState, World};
use serde_json::Value;
use std::rc::Rc;

use crate::world::CakeWorld;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub code: String,
    pub short: String,
    pub long: String,
    pub image_url: String,
    pub data: Rc<Vec<u8>>,
}

pub fn make_give_items(world: &CakeWorld, character: &Option<String>) -> Rc<Vec<Rc<Item>>> {
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
                Rc::new(Item {
                    code: i.name().to_string(),
                    short: i.short(world),
                    long: i.long(world),
                    image_url: format!("images/{}.svg", i.name()),
                    data: Rc::new(serde_json::to_vec(&data).unwrap()),
                })
            })
            .collect()
    } else {
        vec![]
    };
    Rc::new(res)
}
