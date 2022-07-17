use pabitell_lib::{translations::get_message_global, webapp::characters::Character, World};
use std::rc::Rc;

pub fn make_characters(world: &dyn World) -> Rc<Vec<Rc<Character>>> {
    Rc::new(vec![
        Rc::new(Character {
            code: Rc::new(None),
            name: Rc::new("narrator".to_string()),
            character_url: Rc::new("images/book.svg".to_string()),
            short: Rc::new(get_message_global("narrator", world.lang(), None)),
            long: Rc::new(get_message_global("narrator", world.lang(), None)),
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
