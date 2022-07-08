use anyhow::{anyhow, Result};
use pabitell_lib::{
    translations::get_available_locales, Character, Description, Dumpable, Item, ItemState, Named,
    Scene, Tagged, World, WorldBuilder,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    characters, items, scenes,
    translations::{get_message, RESOURCES},
};

const DEFAULT_LANG: &str = "cs";

#[derive(Debug, Default)]
pub struct DollWorld {
    id: Uuid,
    lang: String,
    items: HashMap<String, Box<dyn Item>>,
    characters: HashMap<String, Box<dyn Character>>,
    scenes: HashMap<String, Box<dyn Scene>>,
    event_count: usize,
}

struct DollWorldDescription;
impl Named for DollWorldDescription {
    fn name(&self) -> &'static str {
        "description"
    }
}

impl Description for DollWorldDescription {
    fn long(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-long", world.name()), None)
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-short", world.name()), None)
    }
}

#[derive(Default)]
pub struct DollWorldBuilder {
    items: Vec<Box<dyn Item>>,
    characters: Vec<Box<dyn Character>>,
    scenes: Vec<Box<dyn Scene>>,
}

impl WorldBuilder<DollWorld> for DollWorldBuilder {
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

    fn build(self) -> Result<DollWorld> {
        Ok(DollWorld {
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

    fn make_world() -> Result<DollWorld> {
        Self::default()
            .scene(Box::new(scenes::Home::default()))
            .scene(Box::new(scenes::Walk::default()))
            .scene(Box::new(scenes::DoggieSeach::default()))
            .scene(Box::new(scenes::KitieSeach::default()))
            .character(Box::new(characters::Kitie::default()))
            .character(Box::new(characters::Doggie::default()))
            .item(Box::new(items::Doll::default()))
            .item(Box::new(items::SmallBall::default()))
            .item(Box::new(items::SandMoulds::default()))
            .item(Box::new(items::Beads::default()))
            .item(Box::new(items::ColoredCubes::default()))
            .item(Box::new(items::Crockery::default()))
            .item(Box::new(items::SmallStove::default()))
            .item(Box::new(items::SmallChair::default()))
            .item(Box::new(items::Pictures::default()))
            .item(Box::new(items::Whistle::default()))
            .item(Box::new(items::Spoon::default()))
            .item(Box::new(items::SmallShovel::default()))
            .item(Box::new(items::WoodenHouses::default()))
            .item(Box::new(items::WoodenTrees::default()))
            .item(Box::new(items::WoodenAnimals::default()))
            .item(Box::new(items::Bucket::default()))
            .item(Box::new(items::WateringCan::default()))
            .item(Box::new(items::BuildingCubes::default()))
            .item(Box::new(items::Slippers::default()))
            .item(Box::new(items::Stockings::default()))
            .item(Box::new(items::FairyTaleBook::default()))
            .item(Box::new(items::Hanky::default()))
            .item(Box::new(items::ColoredCloth::default()))
            .item(Box::new(items::Threads::default()))
            .item(Box::new(items::Needlework::default()))
            .item(Box::new(items::RoundNeedle::default()))
            .item(Box::new(items::SmallDoll::default()))
            .item(Box::new(items::FeatherBall::default()))
            .item(Box::new(items::ColoredPapers::default()))
            .item(Box::new(items::ThrowingRing::default()))
            .item(Box::new(items::Cuttlery::default()))
            .build()
    }
}

impl Tagged for DollWorld {}

impl Named for DollWorld {
    fn name(&self) -> &'static str {
        "doggie_and_kitie-doll"
    }
}

impl World for DollWorld {
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
        Box::new(DollWorldDescription)
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
            .for_each(|c| c.set_scene(Some("home".into())));

        self.items_mut().values_mut().for_each(|i| {
            i.set_state(if i.name() == "doll" {
                ItemState::InScene("walk".into())
            } else if i.get_tags().contains(&"doggie_pick".to_owned()) {
                ItemState::InScene("doggie_search".into())
            } else if i.get_tags().contains(&"kitie_pick".to_owned()) {
                ItemState::InScene("kitie_search".into())
            } else {
                ItemState::Unassigned
            })
        });
    }

    fn finished(&self) -> bool {
        self.scenes().get("home").unwrap().dialog() == Some(18)
    }

    fn event_count(&self) -> usize {
        self.event_count
    }

    fn event_inc(&mut self) {
        self.event_count += 1;
    }

    fn extra_clean(&mut self) {
        self.event_count = 0;
    }

    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
    fn get_message(&self, msgid: &str, args: Option<fluent_bundle::FluentArgs>) -> String {
        get_message(msgid, &self.lang, args)
    }
}

impl Dumpable for DollWorld {
    fn dump(&self) -> serde_json::Value {
        serde_json::json!({
            "characters": self.characters.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "items": self.items.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "scenes": self.scenes.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "event_count": self.event_count,
        })
    }
    fn load(&mut self, data: serde_json::Value) -> Result<()> {
        match data {
            // TODO it might be required to check whether all characters, items and scenes exist
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
                        (k, v) if k == "event_count" => {
                            if let serde_json::Value::Number(num) = v {
                                if let Some(count) = num.as_u64() {
                                    self.event_count = count as usize;
                                } else {
                                    return Err(anyhow!(""));
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
