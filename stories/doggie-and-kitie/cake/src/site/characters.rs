use pabitell_lib::{Description, World};

use crate::{translations, world::CakeWorld};

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub code: Option<String>,
    pub name: String,
    pub character_url: String,
    pub short: String,
    pub long: String,
    pub icon: String,
}

pub fn make_characters(world: &CakeWorld) -> Vec<Character> {
    vec![
        Character {
            code: None,
            name: "narrator".to_string(),
            character_url: "svgs/solid/user-secret.svg".to_string(),
            short: translations::get_message("narrator", world.lang(), None),
            long: translations::get_message("narrator", world.lang(), None),
            icon: "fas fa-user-secret".to_string(),
        },
        Character {
            code: Some("doggie".to_string()),
            name: "doggie".to_string(),
            character_url: "svgs/solid/dog.svg".to_string(),
            short: world.characters().get("doggie").unwrap().short(world),
            long: world.characters().get("doggie").unwrap().long(world),
            icon: "fas fa-dog".to_string(),
        },
        Character {
            code: Some("kitie".to_string()),
            name: "kitie".to_string(),
            character_url: "svgs/solid/cat.svg".to_string(),
            short: world.characters().get("kitie").unwrap().short(world),
            long: world.characters().get("kitie").unwrap().long(world),
            icon: "fas fa-cat".to_string(),
        },
    ]
}
