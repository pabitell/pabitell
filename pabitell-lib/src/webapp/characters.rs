use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub code: Rc<Option<String>>,
    pub name: Rc<String>,
    pub character_url: Rc<String>,
    pub short: Rc<String>,
    pub long: Rc<String>,
    pub icon: Rc<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterQRJson {
    pub character: Option<String>,
    pub world_id: Uuid,
}

impl CharacterQRJson {
    pub fn new(character: Option<String>, world_id: Uuid) -> Self {
        Self {
            character,
            world_id,
        }
    }
}
