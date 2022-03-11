use crate::world::DollWorldBuilder;
use pabitell_lib::{World, WorldBuilder};

pub fn make_world(lang: &str) -> Box<dyn World> {
    let mut world = DollWorldBuilder::make_world().unwrap();
    world.setup();
    world.set_lang(lang);
    Box::new(world)
}
