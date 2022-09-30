use crate::{GeoLocation, World};

#[derive(Clone, Debug, PartialEq)]
pub struct Scene {
    pub code: String,
    pub short: String,
    pub image_url: String,
    pub geo_location: Option<GeoLocation>,
}

pub fn make_scenes(world: &dyn World) -> Vec<Scene> {
    world
        .scenes()
        .values()
        .map(|s| Scene {
            code: s.name().to_string(),
            short: s.short(world),
            image_url: format!("images/{}.svg", s.name()),
            geo_location: s.geo_location(),
        })
        .collect()
}
