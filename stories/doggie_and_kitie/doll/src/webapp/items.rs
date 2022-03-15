use pabitell_lib::{data::UseItemData, webapp::items, ItemState, World};
use std::rc::Rc;

use crate::events::ProtocolEvent;

pub fn make_owned_items(world: &dyn World, character: &Option<String>) -> Rc<Vec<Rc<items::Item>>> {
    let res = if let Some(character) = character {
        let owned_state = ItemState::Owned(character.to_string());
        world
            .items()
            .values()
            .filter(|i| i.state() == &owned_state)
            .map(|i| {
                let data = ProtocolEvent::LayDown(UseItemData::new(character, i.name()));
                Rc::new(items::Item {
                    code: i.name().to_string(),
                    short: i.short(world),
                    long: i.long(world),
                    image_url: format!("images/{}.svg", i.name()),
                    give_data: None,
                    use_data: Some(Rc::new(serde_json::to_vec(&data).unwrap())),
                    scan: false,
                    default: items::DefaultAction::Use,
                })
            })
            .collect()
    } else {
        vec![]
    };
    Rc::new(res)
}
