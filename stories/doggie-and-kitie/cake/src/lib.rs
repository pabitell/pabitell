pub mod characters;
pub mod events;
pub mod items;
pub mod narrator;
pub mod scenes;
pub mod translations;

use anyhow::{anyhow, Result};
use pabitell_lib::ItemState;
use pabitell_lib::{
    translations::get_available_locales, Character, Description, Dumpable, Id, Item, Named, Scene,
    Tagged, World, WorldBuilder,
};
use std::{collections::HashMap, convert::TryFrom};
use uuid::Uuid;

use crate::translations::{get_message, RESOURCES};

const DEFAULT_LANG: &str = "cs";

#[derive(Debug, Default)]
pub struct CakeWorld {
    id: Uuid,
    lang: String,
    items: HashMap<String, Box<dyn Item>>,
    characters: HashMap<String, Box<dyn Character>>,
    scenes: HashMap<String, Box<dyn Scene>>,
}

struct CakeWorldDescription;
impl Named for CakeWorldDescription {
    fn name(&self) -> &'static str {
        "description"
    }
}

impl Description for CakeWorldDescription {
    fn long(&self, world: &dyn World) -> String {
        get_message(&format!("{}-long", world.name()), world.lang(), None)
    }

    fn short(&self, world: &dyn World) -> String {
        get_message(&format!("{}-short", world.name()), world.lang(), None)
    }
}

#[derive(Default)]
pub struct CakeWorldBuilder {
    items: Vec<Box<dyn Item>>,
    characters: Vec<Box<dyn Character>>,
    scenes: Vec<Box<dyn Scene>>,
}

impl WorldBuilder<CakeWorld> for CakeWorldBuilder {
    fn character(mut self, character: Box<dyn Character>) -> Self {
        self.characters.push(character);
        self
    }

    fn item(mut self, item: Box<dyn Item>) -> Self {
        self.items.push(item);
        self
    }

    fn scene(mut self, scene: Box<dyn Scene>) -> Self {
        self.scenes.push(scene);
        self
    }

    fn build(self) -> Result<CakeWorld> {
        Ok(CakeWorld {
            lang: DEFAULT_LANG.into(),
            characters: self
                .characters
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),
            items: self
                .items
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),
            scenes: self
                .scenes
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),

            ..Default::default()
        })
    }

    fn make_world() -> Result<CakeWorld> {
        Self::default()
            .scene(Box::new(scenes::PlayGround::default()))
            .scene(Box::new(scenes::Kitchen::default()))
            .scene(Box::new(scenes::Garden::default()))
            .scene(Box::new(scenes::ChildrenHouse::default()))
            .scene(Box::new(scenes::ChildrenGarden::default()))
            .scene(Box::new(scenes::WayHome::default()))
            .character(Box::new(characters::Kitie::default()))
            .character(Box::new(characters::Doggie::default()))
            .item(Box::new(items::SandCake::default()))
            .item(Box::new(items::Flour::default()))
            .item(Box::new(items::Milk::default()))
            .item(Box::new(items::Egg::default()))
            .item(Box::new(items::Butter::default()))
            .item(Box::new(items::Suggar::default()))
            .item(Box::new(items::Salt::default()))
            .item(Box::new(items::Jam::default()))
            .item(Box::new(items::Cheese::default()))
            .item(Box::new(items::Bacon::default()))
            .item(Box::new(items::Peanuts::default()))
            .item(Box::new(items::Cucumber::default()))
            .item(Box::new(items::Bones::default()))
            .item(Box::new(items::FourMice::default()))
            .item(Box::new(items::Sausages::default()))
            .item(Box::new(items::WhippedCream::default()))
            .item(Box::new(items::Onion::default()))
            .item(Box::new(items::Chocolate::default()))
            .item(Box::new(items::Sauce::default()))
            .item(Box::new(items::Garlic::default()))
            .item(Box::new(items::Pepper::default()))
            .item(Box::new(items::Lard::default()))
            .item(Box::new(items::Candy::default()))
            .item(Box::new(items::Greaves::default()))
            .item(Box::new(items::Cinnamon::default()))
            .item(Box::new(items::Porridge::default()))
            .item(Box::new(items::CottageCheese::default()))
            .item(Box::new(items::GingerBread::default()))
            .item(Box::new(items::Vinegar::default()))
            .item(Box::new(items::GooseHead::default()))
            .item(Box::new(items::Cocoa::default()))
            .item(Box::new(items::Cabbadge::default()))
            .item(Box::new(items::Raisins::default()))
            .item(Box::new(items::Bread::default()))
            .item(Box::new(items::Marbles::default()))
            .item(Box::new(items::Ball::default()))
            .item(Box::new(items::Dice::default()))
            .item(Box::new(items::BadDog::default()))
            .item(Box::new(items::Soup::default()))
            .item(Box::new(items::Meat::default()))
            .item(Box::new(items::Dumplings::default()))
            .item(Box::new(items::Pie::default()))
            .build()
    }
}

impl Id for CakeWorld {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
}

impl Tagged for CakeWorld {}

impl Named for CakeWorld {
    fn name(&self) -> &'static str {
        "doggie_and_kitie-cake"
    }
}

impl World for CakeWorld {
    fn characters(&self) -> &HashMap<String, Box<dyn Character>> {
        &self.characters
    }

    fn characters_mut(&mut self) -> &mut HashMap<String, Box<dyn Character>> {
        &mut self.characters
    }

    fn scenes(&self) -> &HashMap<String, Box<dyn Scene>> {
        &self.scenes
    }

    fn scenes_mut(&mut self) -> &mut HashMap<String, Box<dyn Scene>> {
        &mut self.scenes
    }

    fn items(&self) -> &HashMap<String, Box<dyn Item>> {
        &self.items
    }

    fn items_mut(&mut self) -> &mut HashMap<String, Box<dyn Item>> {
        &mut self.items
    }

    fn description(&self) -> Box<dyn Description> {
        Box::new(CakeWorldDescription)
    }

    fn lang(&self) -> &str {
        &self.lang
    }

    fn set_lang(&mut self, lang: &str) -> bool {
        if let Ok(locales) = get_available_locales(&RESOURCES) {
            if locales.iter().any(|l| l.to_string() == lang) {
                self.lang = lang.into();
                return true;
            }
        }
        false
    }

    fn available_languages(&self) -> Vec<&str> {
        vec!["cs", "en-US"]
    }

    fn setup(&mut self) {
        self.randomize_ids();

        self.characters_mut()
            .values_mut()
            .for_each(|c| c.set_scene(Some("playground".into())));

        self.items_mut().values_mut().for_each(|i| {
            i.set_state(match i.name() {
                "sand_cake" => ItemState::InScene("playground".into()),
                "bad_dog" => ItemState::InScene("garden".into()),
                _ => {
                    if i.get_tags().contains(&"ingredient".to_string()) {
                        ItemState::InScene("kitchen".into())
                    } else if i.get_tags().contains(&"toy".to_string()) {
                        ItemState::InScene("children_garden".into())
                    } else if i.get_tags().contains(&"meal".to_string()) {
                        ItemState::InScene("children_house".into())
                    } else {
                        ItemState::Unassigned
                    }
                }
            })
        });
    }

    fn finished(&self) -> bool {
        // test if doggie and kitie are ready to go
        let doggie = self
            .characters()
            .get("doggie")
            .unwrap()
            .as_any()
            .downcast_ref::<characters::Doggie>()
            .unwrap();

        let kitie = self
            .characters()
            .get("kitie")
            .unwrap()
            .as_any()
            .downcast_ref::<characters::Kitie>()
            .unwrap();

        doggie.scene().clone() == Some("way_home".to_string())
            && kitie.scene().clone() == Some("way_home".to_string())
    }
}

impl Dumpable for CakeWorld {
    fn dump(&self) -> serde_json::Value {
        serde_json::json!({
            "characters": self.characters.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "items": self.items.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "scenes": self.scenes.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
        })
    }
    fn load(&mut self, data: serde_json::Value) -> Result<()> {
        match data {
            // TODO it might be required to check whether all characters, itemsand scenes exist
            // before loading data
            serde_json::Value::Object(root) => {
                for item in root {
                    match item {
                        (k, v) if k == "characters" => {
                            if let serde_json::Value::Object(characters) = v {
                                for (name, data) in characters.into_iter() {
                                    let character = self
                                        .characters_mut()
                                        .get_mut(&name)
                                        .ok_or_else(|| anyhow!("missing character '{}'", name))?;
                                    character.load(data)?;
                                }
                            } else {
                                return Err(anyhow!(""));
                            }
                        }
                        (k, v) if k == "items" => {
                            if let serde_json::Value::Object(items) = v {
                                for (name, data) in items.into_iter() {
                                    let item = self
                                        .items_mut()
                                        .get_mut(&name)
                                        .ok_or_else(|| anyhow!("missing item '{}'", name))?;
                                    item.load(data)?;
                                }
                            } else {
                                return Err(anyhow!(""));
                            }
                        }
                        (k, v) if k == "scenes" => {
                            if let serde_json::Value::Object(scenes) = v {
                                for (name, data) in scenes.into_iter() {
                                    let scene = self
                                        .scenes_mut()
                                        .get_mut(&name)
                                        .ok_or_else(|| anyhow!("missing scene '{}'", name))?;
                                    scene.load(data)?;
                                }
                            } else {
                                return Err(anyhow!(""));
                            }
                        }
                        _ => return Err(anyhow!("")),
                    }
                }
            }
            _ => return Err(anyhow!("")),
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use pabitell_lib::{
        events as lib_events, Description, Dumpable, Event, Id, ItemState, Narrator, World,
        WorldBuilder,
    };
    use uuid::Uuid;

    use super::events;
    use crate::{characters, narrator, CakeWorld, CakeWorldBuilder};

    pub fn prepare_world() -> CakeWorld {
        let mut world = CakeWorldBuilder::make_world().unwrap();
        world.setup();
        world
    }

    #[test]
    fn setup() {
        let world = prepare_world();
        assert_eq!(
            world.characters().get("kitie").unwrap().scene(),
            &Some("playground".into())
        );
        assert_eq!(
            world.characters().get("doggie").unwrap().scene(),
            &Some("playground".into())
        );
        assert_eq!(
            world.items().get("sand_cake").unwrap().state(),
            &ItemState::InScene("playground".into())
        );
        assert_eq!(
            world.items().get("milk").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("jam").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("bread").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("raisins").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("ball").unwrap().state(),
            &ItemState::InScene("children_garden".into())
        );
        assert_eq!(
            world.items().get("bad_dog").unwrap().state(),
            &ItemState::InScene("garden".into())
        );
        assert_eq!(
            world.items().get("dumplings").unwrap().state(),
            &ItemState::InScene("children_house".into())
        );
    }

    fn reload_world(world: CakeWorld) -> CakeWorld {
        let dump = world.dump();
        let mut new_world = CakeWorldBuilder::make_world().unwrap();
        dbg!(new_world.load(dump)).unwrap();
        assert_eq!(new_world.dump(), world.dump());
        new_world
    }

    #[test]
    fn workflow() {
        let mut world = prepare_world();
        world = reload_world(world);

        let narrator = narrator::Cake::default();

        // pick sand cake
        let mut events = narrator.available_events(&world);
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|e| e.name() == "pick"));
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        assert!(!events[1].can_be_triggered(&world));
        assert!(!events[1].perform(&mut world));
        world = reload_world(world);

        // give and consume sand cake
        let mut events = narrator.available_events(&world);
        dbg!(&events);
        assert_eq!(events.len(), 1);
        assert!(events.iter().all(|e| e.name() == "give_sand_cake"));
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        world = reload_world(world);

        // move both characters to kitchen
        let mut events = narrator.available_events(&world);
        assert_eq!(events.len(), 2);
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        world = reload_world(world);
        let mut events = narrator.available_events(&world);
        assert_eq!(events.len(), 1);
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        world = reload_world(world);

        let mut doggie = false;
        let mut events = narrator.available_events(&world);
        for event in narrator.available_events(&world).iter_mut() {
            if event.get_tags().contains(&"pick".to_string()) {
                // Put thinkgs to cake
                let cevent = event
                    .as_any_mut()
                    .downcast_mut::<lib_events::Pick>()
                    .unwrap();
                if cevent.character() == "kitie" {
                    assert!(cevent.can_be_triggered(&world));
                    assert!(cevent.perform(&mut world));
                    world = reload_world(world);
                }
            } else if event.get_tags().contains(&"void".to_string()) {
                // Put disliked thing to cake
                let cevent = event
                    .as_any_mut()
                    .downcast_mut::<lib_events::Void>()
                    .unwrap();
                assert!(cevent.can_be_triggered(&world));
                assert!(cevent.perform(&mut world));
                world = reload_world(world);
            }
        }

        for event in narrator.available_events(&world).iter_mut() {
            if event.get_tags().contains(&"use_item".to_string()) {
                assert!(event.can_be_triggered(&world));
                assert!(event.perform(&mut world));
                world = reload_world(world);
            }
        }

        // move both characters to children's garden
        for event in narrator.available_events(&world).iter_mut() {
            if event.get_tags().contains(&"move".to_string()) {
                assert!(event.can_be_triggered(&world));
                assert!(event.perform(&mut world));
                world = reload_world(world);
            }
        }

        // play with children
        for event in narrator.available_events(&world).iter_mut() {
            let cevent = event
                .as_any_mut()
                .downcast_mut::<lib_events::Pick>()
                .unwrap();
            if cevent.character() == "doggie" {
                assert!(cevent.can_be_triggered(&world));
                assert!(cevent.perform(&mut world));
                world = reload_world(world);
            }
        }

        // move to garden
        for event in narrator.available_events(&world).iter_mut() {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            world = reload_world(world);
        }

        // find big bad dog
        let mut events = narrator.available_events(&world);
        assert_eq!(events.len(), 2);
        let mut event = events.remove(0);
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        world = reload_world(world);

        // go to children house
        let mut events = narrator.available_events(&world);
        assert_eq!(events.len(), 2);
        for event in narrator.available_events(&world).iter_mut() {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            world = reload_world(world);
        }

        let mut events = narrator.available_events(&world);
        assert_eq!(events.len(), 8);
        for event in events.iter_mut() {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            world = reload_world(world);
        }

        // Make sure that doggie and kitie reached final destination
        assert!(world.characters().get("doggie").unwrap().scene() == &Some("way_home".into()));
        assert!(world.characters().get("kitie").unwrap().scene() == &Some("way_home".into()));

        assert!(world.finished());
    }

    #[test]
    fn languages() {
        let mut world = CakeWorldBuilder::make_world().unwrap();
        for lang in vec!["cs", "en-US"] {
            assert!(world.set_lang(lang));
        }
    }
}
