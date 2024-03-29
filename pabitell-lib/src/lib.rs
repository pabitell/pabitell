#[cfg(feature = "with_cli")]
pub mod cli;
pub mod conditions;
pub mod data;
pub mod events;
pub mod items;
pub mod protocol;
pub mod scenes;
pub mod translations;
pub mod updates;
#[cfg(feature = "with_webapp")]
pub mod webapp;

use anyhow::{anyhow, Result};
use fluent::FluentArgs;
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap, fmt};
use uuid::Uuid;

use conditions::Check;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ItemState {
    Owned(String),
    InScene(String),
    Unassigned,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct GeoLocation(f64, f64);

impl Default for ItemState {
    fn default() -> Self {
        Self::Unassigned
    }
}

impl Dumpable for ItemState {
    fn dump(&self) -> serde_json::Value {
        match self {
            Self::Owned(character) => serde_json::json!({"kind": "character", "value": character}),
            Self::InScene(scene) => serde_json::json!({"kind": "scene", "value": scene}),
            Self::Unassigned => serde_json::Value::Null,
        }
    }
    fn load(&mut self, data: serde_json::Value) -> Result<()> {
        let new = match data {
            serde_json::Value::Null => Self::Unassigned,
            serde_json::Value::Object(mut inner) => {
                let value = if let serde_json::Value::String(value) = inner
                    .remove("value")
                    .ok_or_else(|| anyhow!("missing value while parsing ItemState"))?
                {
                    value
                } else {
                    return Err(anyhow!("Mismatched type in ItemValue"));
                };
                match inner
                    .remove("kind")
                    .ok_or_else(|| anyhow!("missing kind while parsing ItemState"))?
                {
                    serde_json::Value::String(kind) if kind == "character" => Self::Owned(value),
                    serde_json::Value::String(kind) if kind == "scene" => Self::InScene(value),
                    e => return Err(anyhow!("Unknown kind `{}`", e)),
                }
            }
            _ => return Err(anyhow!("Failed to deserialize ItemState")),
        };
        *self = new;

        Ok(())
    }
}

pub trait Tagged {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
    fn set_tags(&mut self, _tags: Vec<String>) {}
}

pub trait Named {
    /// unique name within world
    fn name(&self) -> &'static str;
}

pub trait Music {
    /// Name of background music file
    fn music(&self) -> Option<String> {
        None
    }
}

pub trait Clean {
    /// Reset to original state
    fn clean(&mut self) {}
}

pub trait Description: Named {
    fn long(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-long", world.name(), self.name()), None)
    }
    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-{}-short", world.name(), self.name()), None)
    }
}

pub trait Dumpable {
    fn dump(&self) -> serde_json::Value;
    fn load(&mut self, data: serde_json::Value) -> Result<()>;
}

pub trait Item: Named + Tagged + AsAny + Description + Dumpable + fmt::Debug + Clean {
    fn state(&self) -> &ItemState;
    fn set_state(&mut self, state: ItemState);
    fn last_event(&self) -> Option<usize>;
    fn set_last_event(&mut self, event: usize);
}

pub trait Character: Named + Tagged + AsAny + Description + Dumpable + fmt::Debug + Clean {
    fn scene(&self) -> &Option<String>;
    fn set_scene(&mut self, scene: Option<String>);
}

pub trait Scene:
    Named + Tagged + AsAny + Description + Dumpable + Music + fmt::Debug + Clean
{
    fn dialog(&self) -> Option<usize> {
        None
    }
    fn next_dialog(&mut self) {}
    fn geo_location(&self) -> Option<GeoLocation> {
        None
    }
    fn set_geo_location(&mut self, _location: Option<GeoLocation>) {}
}

pub trait Event: Tagged + AsAny + fmt::Debug + PartialEq<[u8]> {
    fn kind(&self) -> &str {
        std::any::type_name::<Self>()
            .rsplitn(2, "::")
            .collect::<Vec<&str>>()[0]
    }
    fn name(&self) -> &str;
    fn can_be_triggered(&self, world: &dyn World) -> bool {
        self.get_condition().check(world).unwrap()
    }
    fn trigger(&mut self, world: &mut dyn World) {
        for update in self.get_world_updates() {
            update.change(world).unwrap();
            for item in self.items() {
                let last_event = world.event_count();
                let item = world.items_mut().get_mut(&item).unwrap();
                item.set_last_event(last_event);
            }
        }
        world.event_inc();
    }
    fn perform(&mut self, world: &mut dyn World) -> bool {
        if self.can_be_triggered(world) {
            self.trigger(world);
            true
        } else {
            false
        }
    }
    fn action_text(&self, world: &dyn World) -> String {
        let msg = format!("{}-action", self.msg_base(world));
        world.get_message(&msg, None)
    }
    fn success_text(&self, world: &dyn World) -> String {
        let msg = format!("{}-success", self.msg_base(world));
        world.get_message(&msg, None)
    }
    fn fail_text(&self, world: &dyn World) -> String {
        let msg = format!("{}-fail", self.msg_base(world));
        world.get_message(&msg, None)
    }
    fn set_world_updates(&mut self, updates: Vec<Box<dyn updates::Change>>);
    fn set_condition(&mut self, condition: conditions::Condition);

    fn msg_base(&self, world: &dyn World) -> String {
        format!("{}-{}", world.name(), self.name())
    }

    fn get_world_updates(&self) -> &[Box<dyn updates::Change>];
    fn get_condition(&self) -> &conditions::Condition;

    fn initiator(&self) -> String;
    fn set_initiator(&mut self, initiator: String);
    fn dump(&self) -> serde_json::Value;
    fn matches(&self, value: &serde_json::Value) -> bool;

    fn items(&self) -> Vec<String>;
    fn characters(&self) -> Vec<String>;

    fn sort_key(&self, world: &dyn World) -> (usize, String, String, String) {
        let max_event_idx = self
            .items()
            .iter()
            .map(|e| {
                if let Some(item) = world.items().get(e) {
                    item.last_event().unwrap_or(0)
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        let max_item_name = self.items().into_iter().max().unwrap_or_default();
        (
            max_event_idx,
            max_item_name,
            self.characters().into_iter().max().unwrap_or_default(),
            self.name().to_string(),
        )
    }

    /// Can be event be triggered by moving to given geo location
    fn geo_location(&self, _world: &dyn World) -> Option<(String, Option<String>, GeoLocation)> {
        None
    }
}

pub trait WorldBuilder<S>
where
    S: World,
{
    fn character(self, character: Box<dyn Character>) -> Self;
    fn item(self, item: Box<dyn Item>) -> Self;
    fn scene(self, scene: Box<dyn Scene>) -> Self;
    fn build(self) -> Result<S>;
    fn make_world() -> Result<S>;
}

pub trait World: Named + Dumpable + Clean {
    fn available_languages(&self) -> Vec<String>;
    fn lang(&self) -> &str;
    fn set_lang(&mut self, lang: &str) -> bool;
    fn description(&self) -> Box<dyn Description>;
    fn scenes(&self) -> &HashMap<String, Box<dyn Scene>>;
    fn scenes_mut(&mut self) -> &mut HashMap<String, Box<dyn Scene>>;
    fn characters(&self) -> &HashMap<String, Box<dyn Character>>;
    fn characters_mut(&mut self) -> &mut HashMap<String, Box<dyn Character>>;
    fn items(&self) -> &HashMap<String, Box<dyn Item>>;
    fn items_mut(&mut self) -> &mut HashMap<String, Box<dyn Item>>;
    fn setup(&mut self, new_id: bool);
    fn reset(&mut self) {
        self.clean_world();
        self.setup(false);
    }
    fn clean_world(&mut self) {
        self.clean();
        self.characters_mut().values_mut().for_each(|e| e.clean());
        self.items_mut().values_mut().for_each(|e| e.clean());
        self.scenes_mut().values_mut().for_each(|e| e.clean());
    }
    fn randomize_id(&mut self) {
        self.set_id(Uuid::new_v4());
    }
    fn finished(&self) -> bool;
    fn event_count(&self) -> usize;
    fn event_inc(&mut self);

    fn id(&self) -> &Uuid;
    fn set_id(&mut self, id: Uuid);
    fn get_message(&self, msgid: &str, _args: Option<FluentArgs>) -> String {
        msgid.to_string()
    }

    /// Format of the world
    ///
    /// when the format of stored world changes
    /// this number should be bumped as well
    fn version(&self) -> usize {
        1
    }
}

pub trait Narrator {
    fn all_events(&self, world: &dyn World) -> Vec<Box<dyn Event>>;
    fn available_events(&self, world: &dyn World) -> Vec<Box<dyn Event>> {
        self.all_events(world)
            .into_iter()
            .filter(|e| e.can_be_triggered(world))
            .collect()
    }
    fn parse_event(&self, world: &dyn World, value: serde_json::Value) -> Option<Box<dyn Event>>;
    fn available_events_sorted(&self, world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut events = self.available_events(world);
        events.sort_by_key(|e| e.sort_key(world));
        events
    }
}

#[cfg(test)]
pub mod test {
    use super::{
        conditions, updates, AsAny, Character, Clean, Description, Dumpable, Event, Item,
        ItemState, Music, Named, Scene, Tagged, World, WorldBuilder,
    };
    use anyhow::{anyhow, Result};
    use std::{any::Any, collections::HashMap};
    use uuid::Uuid;

    #[derive(Debug, Default)]
    struct TestCharacter {
        scene: Option<String>,
    }

    impl Tagged for TestCharacter {}

    impl Named for TestCharacter {
        fn name(&self) -> &'static str {
            "test_character"
        }
    }

    impl Description for TestCharacter {}

    impl AsAny for TestCharacter {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Clean for TestCharacter {
        fn clean(&mut self) {}
    }

    impl Character for TestCharacter {
        fn scene(&self) -> &Option<String> {
            &self.scene
        }
        fn set_scene(&mut self, scene: Option<String>) {
            self.scene = scene
        }
    }

    impl Dumpable for TestCharacter {
        fn dump(&self) -> serde_json::Value {
            serde_json::json!(
                {
                    "name": self.name(),
                }
            )
        }

        fn load(&mut self, _data: serde_json::Value) -> Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, Default)]
    struct TestItem {
        state: ItemState,
        last_event: Option<usize>,
    }

    impl Tagged for TestItem {}

    impl Named for TestItem {
        fn name(&self) -> &'static str {
            "test_item"
        }
    }

    impl Clean for TestItem {
        fn clean(&mut self) {}
    }

    impl Description for TestItem {}

    impl AsAny for TestItem {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Dumpable for TestItem {
        fn dump(&self) -> serde_json::Value {
            serde_json::json!(
                {
                    "name": self.name(),
                }
            )
        }

        fn load(&mut self, _data: serde_json::Value) -> Result<()> {
            Ok(())
        }
    }

    impl Item for TestItem {
        fn state(&self) -> &ItemState {
            &self.state
        }
        fn set_state(&mut self, state: ItemState) {
            self.state = state;
        }
        fn last_event(&self) -> Option<usize> {
            self.last_event
        }
        fn set_last_event(&mut self, event: usize) {
            self.last_event = Some(event);
        }
    }

    #[derive(Debug, Default)]
    struct TestScene {}

    impl Tagged for TestScene {}

    impl Named for TestScene {
        fn name(&self) -> &'static str {
            "test_scene"
        }
    }

    impl Description for TestScene {}

    impl Clean for TestScene {
        fn clean(&mut self) {}
    }

    impl AsAny for TestScene {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Dumpable for TestScene {
        fn dump(&self) -> serde_json::Value {
            serde_json::json!(
                {
                    "name": self.name(),
                }
            )
        }

        fn load(&mut self, _data: serde_json::Value) -> Result<()> {
            Ok(())
        }
    }

    impl Music for TestScene {}

    impl Scene for TestScene {}

    #[derive(Clone, Debug)]
    struct TestDescription;
    impl Named for TestDescription {
        fn name(&self) -> &'static str {
            "test_description"
        }
    }

    impl Description for TestDescription {}

    #[derive(Debug)]
    struct TestEvent {
        #[allow(dead_code)]
        description: TestDescription,
        condition: conditions::Condition,
    }
    impl Tagged for TestEvent {}
    impl Named for TestEvent {
        fn name(&self) -> &'static str {
            "test_event"
        }
    }
    impl AsAny for TestEvent {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
    impl PartialEq<[u8]> for TestEvent {
        fn eq(&self, _other: &[u8]) -> bool {
            false
        }
    }
    impl Event for TestEvent {
        fn trigger(&mut self, _world: &mut dyn World) {}

        fn can_be_triggered(&self, _world: &dyn World) -> bool {
            true
        }

        fn name(&self) -> &str {
            "test_event"
        }

        fn action_text(&self, _: &dyn World) -> String {
            "action".into()
        }

        fn success_text(&self, _: &dyn World) -> String {
            "success".into()
        }

        fn fail_text(&self, _: &dyn World) -> String {
            "fail".into()
        }

        fn set_world_updates(&mut self, _updates: Vec<Box<dyn updates::Change>>) {}
        fn set_condition(&mut self, _condition: conditions::Condition) {}
        fn get_world_updates(&self) -> &[Box<dyn updates::Change>] {
            &[]
        }
        fn get_condition(&self) -> &conditions::Condition {
            &self.condition
        }

        fn initiator(&self) -> String {
            "test_character".into()
        }

        fn set_initiator(&mut self, _initiator: String) {}

        fn dump(&self) -> serde_json::Value {
            serde_json::json!({})
        }

        fn matches(&self, _value: &serde_json::Value) -> bool {
            false
        }

        fn items(&self) -> Vec<String> {
            vec![]
        }

        fn characters(&self) -> Vec<String> {
            vec!["test_character".to_owned()]
        }
    }

    #[derive(Debug, Default)]
    struct TestWorld {
        id: Uuid,
        lang: String,
        items: HashMap<String, Box<dyn Item>>,
        scenes: HashMap<String, Box<dyn Scene>>,
        characters: HashMap<String, Box<dyn Character>>,
        event_count: usize,
    }

    impl Named for TestWorld {
        fn name(&self) -> &'static str {
            "test_world"
        }
    }

    impl Clean for TestWorld {
        fn clean(&mut self) {}
    }

    impl World for TestWorld {
        fn description(&self) -> Box<dyn Description> {
            Box::new(TestDescription)
        }

        fn scenes(&self) -> &HashMap<String, Box<dyn Scene>> {
            &self.scenes
        }

        fn scenes_mut(&mut self) -> &mut HashMap<String, Box<dyn Scene>> {
            &mut self.scenes
        }

        fn characters(&self) -> &HashMap<String, Box<dyn Character>> {
            &self.characters
        }

        fn characters_mut(&mut self) -> &mut HashMap<String, Box<dyn Character>> {
            &mut self.characters
        }

        fn items(&self) -> &HashMap<String, Box<dyn Item>> {
            &self.items
        }

        fn items_mut(&mut self) -> &mut HashMap<String, Box<dyn Item>> {
            &mut self.items
        }

        fn lang(&self) -> &str {
            &self.lang
        }

        fn set_lang(&mut self, lang: &str) -> bool {
            self.lang = lang.into();
            true
        }

        fn available_languages(&self) -> Vec<String> {
            vec!["en-US".to_string()]
        }

        fn setup(&mut self, _new_id: bool) {}

        fn finished(&self) -> bool {
            true
        }

        fn event_count(&self) -> usize {
            self.event_count
        }

        fn event_inc(&mut self) {
            self.event_count += 1;
        }

        fn id(&self) -> &Uuid {
            &self.id
        }

        fn set_id(&mut self, id: Uuid) {
            self.id = id
        }
    }

    impl Dumpable for TestWorld {
        fn dump(&self) -> serde_json::Value {
            serde_json::json!({
                "characters": self.characters.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
                "items": self.items.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
                "scenes": self.scenes.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
                "event_count": self.event_count
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
                                            .ok_or_else(|| anyhow!(""))?;
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
                                            .characters_mut()
                                            .get_mut(&name)
                                            .ok_or_else(|| anyhow!(""))?;
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
                                            .characters_mut()
                                            .get_mut(&name)
                                            .ok_or_else(|| anyhow!(""))?;
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

    #[derive(Default)]
    struct TestWorldBuilder {
        characters: Vec<Box<dyn Character>>,
        items: Vec<Box<dyn Item>>,
        scenes: Vec<Box<dyn Scene>>,
    }

    impl WorldBuilder<TestWorld> for TestWorldBuilder {
        fn character(mut self, character: Box<dyn Character>) -> Self {
            self.characters.push(character);
            self
        }

        fn item(mut self, item: Box<dyn Item>) -> Self {
            self.items.push(item);
            self
        }

        fn scene(mut self, item: Box<dyn Scene>) -> Self {
            self.scenes.push(item);
            self
        }

        fn build(self) -> Result<TestWorld> {
            Ok(TestWorld {
                lang: "en-US".into(),
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

        fn make_world() -> Result<TestWorld> {
            Self::default().build()
        }
    }

    #[test]
    fn linear() {
        let world = TestWorldBuilder::default()
            .character(Box::new(TestCharacter::default()))
            .character(Box::new(TestCharacter::default()))
            .item(Box::new(TestItem::default()))
            .item(Box::new(TestItem::default()))
            .item(Box::new(TestItem::default()))
            .item(Box::new(TestItem::default()))
            .scene(Box::new(TestScene::default()))
            .scene(Box::new(TestScene::default()))
            .scene(Box::new(TestScene::default()));

        assert!(world.build().is_ok());
    }
}
