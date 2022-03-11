use crate::narrator::Doll;
use pabitell_lib::Narrator;

pub fn make_narrator() -> Box<dyn Narrator> {
    Box::new(Doll::default())
}
