use pabitell_lib::{Description, World};
use std::rc::Rc;

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

pub fn make_characters(world: &CakeWorld) -> Vec<Rc<Character>> {
    vec![
        Rc::new(Character {
            code: Rc::new(None),
            name: Rc::new("narrator".to_string()),
            character_url: Rc::new("svgs/solid/user-secret.svg".to_string()),
            short: Rc::new(translations::get_message("narrator", world.lang(), None)),
            long: Rc::new(translations::get_message("narrator", world.lang(), None)),
            icon: Rc::new("fas fa-user-secret".to_string()),
        }),
        Rc::new(Character {
            code: Rc::new(Some("doggie".to_string())),
            name: Rc::new("doggie".to_string()),
            character_url: Rc::new("svgs/solid/dog.svg".to_string()),
            short: Rc::new(world.characters().get("doggie").unwrap().short(world)),
            long: Rc::new(world.characters().get("doggie").unwrap().long(world)),
            icon: Rc::new("fas fa-dog".to_string()),
        }),
        Rc::new(Character {
            code: Rc::new(Some("kitie".to_string())),
            name: Rc::new("kitie".to_string()),
            character_url: Rc::new("svgs/solid/cat.svg".to_string()),
            short: Rc::new(world.characters().get("kitie").unwrap().short(world)),
            long: Rc::new(world.characters().get("kitie").unwrap().long(world)),
            icon: Rc::new("fas fa-cat".to_string()),
        }),
    ]
}
