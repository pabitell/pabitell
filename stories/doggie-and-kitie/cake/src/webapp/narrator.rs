use crate::narrator::Cake;
use pabitell_lib::Narrator;

pub fn make_narrator() -> Box<dyn Narrator> {
    Box::new(Cake::default())
}
