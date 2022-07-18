use pabitell_lib::{data::GiveData, webapp::items, ItemState, World};
use std::rc::Rc;

use crate::events::ProtocolEvent;

pub fn make_owned_items(world: &dyn World, character: &Option<String>) -> Vec<Rc<items::Item>> {
    if let Some(character) = character {
        let owned_state = ItemState::Owned(character.to_string());
        world
            .items()
            .values()
            .filter(|i| i.state() == &owned_state)
            .map(|i| {
                let data = if i.name() == "sand_cake" {
                    ProtocolEvent::GiveSandCake(GiveData::new(
                        character,
                        String::new(), // keep target character empty
                        i.name(),
                    ))
                } else {
                    ProtocolEvent::Give(GiveData::new(
                        character,
                        String::new(), // keep target character empty
                        i.name(),
                    ))
                };
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
                    last_event: i.last_event(),
                })
            })
            .collect()
    } else {
        vec![]
    }
}
