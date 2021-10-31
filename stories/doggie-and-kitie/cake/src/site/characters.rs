use pabitell_lib::{Description, World};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

use crate::{translations, world::CakeWorld};

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
    pub character: String,
    pub world_id: Uuid,
}

impl CharacterQRJson {
    pub fn new(character: String, world_id: Uuid) -> Self {
        Self {
            character,
            world_id,
        }
    }
}

pub fn make_characters(world: &CakeWorld) -> Rc<Vec<Rc<Character>>> {
    Rc::new(vec![
        Rc::new(Character {
            code: Rc::new(None),
            name: Rc::new("narrator".to_string()),
            character_url: Rc::new("images/book.svg".to_string()),
            short: Rc::new(translations::get_message("narrator", world.lang(), None)),
            long: Rc::new(translations::get_message("narrator", world.lang(), None)),
            icon: Rc::new("fas fa-book".to_string()),
        }),
        Rc::new(Character {
            code: Rc::new(Some("doggie".to_string())),
            name: Rc::new("doggie".to_string()),
            character_url: Rc::new("images/dog.svg".to_string()),
            short: Rc::new(world.characters().get("doggie").unwrap().short(world)),
            long: Rc::new(world.characters().get("doggie").unwrap().long(world)),
            icon: Rc::new("fas fa-dog".to_string()),
        }),
        Rc::new(Character {
            code: Rc::new(Some("kitie".to_string())),
            name: Rc::new("kitie".to_string()),
            character_url: Rc::new("images/cat.svg".to_string()),
            short: Rc::new(world.characters().get("kitie").unwrap().short(world)),
            long: Rc::new(world.characters().get("kitie").unwrap().long(world)),
            icon: Rc::new("fas fa-cat".to_string()),
        }),
    ])
}
