pub mod events;
#[cfg(feature = "with_translations")]
pub mod translations;

use anyhow::Result;
use std::{any::Any, collections::HashMap, fmt};
use uuid::Uuid;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, PartialEq)]
pub enum ItemState {
    Owned(String),
    InScene(String),
    Unassigned,
}

impl Default for ItemState {
    fn default() -> Self {
        Self::Unassigned
    }
}

pub trait Id {
    fn id(&self) -> &Uuid;
    fn set_id(&mut self, id: Uuid);
    /// unique name within world
    fn roles(&self) -> Vec<&'static str>;
}

pub trait Named {
    /// unique name within world
    fn name(&self) -> &'static str;
}

pub trait Description: Named {
    fn long(&self, world: &dyn World) -> String;
    fn short(&self, world: &dyn World) -> String;
}

pub trait Item: Id + Named + AsAny + Description + fmt::Debug {
    fn state(&self) -> &ItemState;
    fn set_state(&mut self, state: ItemState);
}

pub trait Character: Id + Named + AsAny + Description + fmt::Debug {
    fn scene(&self) -> &Option<String>;
    fn set_scene(&mut self, scene: Option<String>);
}

pub trait Scene: Id + Named + AsAny + Description + fmt::Debug {}

pub trait Event: Id + AsAny + fmt::Debug {
    fn name(&self) -> &str;
    fn can_be_triggered(&self, world: &dyn World) -> bool;
    fn trigger(&mut self, world: &mut dyn World);
    fn perform(&mut self, world: &mut dyn World) -> bool {
        if self.can_be_triggered(world) {
            self.trigger(world);
            true
        } else {
            false
        }
    }
    fn translation_base(&self) -> String;
    fn action_text(&self, world: &dyn World) -> String;
    fn success_text(&self, world: &dyn World) -> String;
    fn fail_text(&self, world: &dyn World) -> String;
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

pub trait World: Id + Named {
    fn available_languages(&self) -> Vec<&str>;
    fn lang(&self) -> &str;
    fn set_lang(&mut self, lang: &str) -> bool;
    fn description(&self) -> Box<dyn Description>;
    fn scenes(&self) -> &HashMap<String, Box<dyn Scene>>;
    fn scenes_mut(&mut self) -> &mut HashMap<String, Box<dyn Scene>>;
    fn characters(&self) -> &HashMap<String, Box<dyn Character>>;
    fn characters_mut(&mut self) -> &mut HashMap<String, Box<dyn Character>>;
    fn items(&self) -> &HashMap<String, Box<dyn Item>>;
    fn items_mut(&mut self) -> &mut HashMap<String, Box<dyn Item>>;
    #[cfg(feature = "with_world_setup")]
    fn setup(&mut self);
    #[cfg(feature = "with_world_setup")]
    fn randomize_ids(&mut self) {
        self.set_id(Uuid::new_v4());

        self.characters_mut()
            .values_mut()
            .for_each(|e| e.set_id(Uuid::new_v4()));

        self.items_mut()
            .values_mut()
            .for_each(|e| e.set_id(Uuid::new_v4()));

        self.scenes_mut()
            .values_mut()
            .for_each(|e| e.set_id(Uuid::new_v4()));
    }
    fn finished(&self) -> bool;
}

pub trait Narrator {
    fn available_events(&self, world: &dyn World) -> Vec<Box<dyn Event>>;
}

#[cfg(test)]
pub mod test {
    use super::{
        AsAny, Character, Description, Event, Id, Item, ItemState, Named, Scene, World,
        WorldBuilder,
    };
    use anyhow::Result;
    use std::{any::Any, collections::HashMap};
    use uuid::Uuid;

    #[derive(Debug, Default)]
    struct TestCharacter {
        id: Uuid,
        scene: Option<String>,
    }

    impl Id for TestCharacter {
        fn id(&self) -> &Uuid {
            &self.id
        }
        fn set_id(&mut self, id: Uuid) {
            self.id = id
        }
        fn roles(&self) -> Vec<&'static str> {
            vec![]
        }
    }

    impl Named for TestCharacter {
        fn name(&self) -> &'static str {
            "test_character"
        }
    }

    impl Description for TestCharacter {
        fn short(&self, _: &dyn World) -> String {
            "Test Character".into()
        }
        fn long(&self, _: &dyn World) -> String {
            "Character description and perhaps items which it is carrying".into()
        }
    }

    impl AsAny for TestCharacter {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Character for TestCharacter {
        fn scene(&self) -> &Option<String> {
            &self.scene
        }
        fn set_scene(&mut self, scene: Option<String>) {
            self.scene = scene
        }
    }

    #[derive(Debug, Default)]
    struct TestItem {
        id: uuid::Uuid,
        state: ItemState,
    }

    impl Id for TestItem {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
        fn set_id(&mut self, id: Uuid) {
            self.id = id
        }
        fn roles(&self) -> Vec<&'static str> {
            vec![]
        }
    }

    impl Named for TestItem {
        fn name(&self) -> &'static str {
            "test_item"
        }
    }

    impl Description for TestItem {
        fn short(&self, _: &dyn World) -> String {
            "Test Scene".into()
        }
        fn long(&self, _: &dyn World) -> String {
            "Test scene detailed description".into()
        }
    }

    impl AsAny for TestItem {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Item for TestItem {
        fn state(&self) -> &ItemState {
            &self.state
        }
        fn set_state(&mut self, state: ItemState) {
            self.state = state;
        }
    }

    #[derive(Debug, Default)]
    struct TestScene {
        id: Uuid,
    }

    impl Id for TestScene {
        fn id(&self) -> &Uuid {
            &self.id
        }
        fn set_id(&mut self, id: Uuid) {
            self.id = id
        }
        fn roles(&self) -> Vec<&'static str> {
            vec![]
        }
    }

    impl Named for TestScene {
        fn name(&self) -> &'static str {
            "test_scene"
        }
    }

    impl Description for TestScene {
        fn short(&self, _: &dyn World) -> String {
            "Test Scene".into()
        }
        fn long(&self, _: &dyn World) -> String {
            "Test location detailed description".into()
        }
    }

    impl AsAny for TestScene {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Scene for TestScene {}

    #[derive(Clone, Debug)]
    struct TestDescription;
    impl Named for TestDescription {
        fn name(&self) -> &'static str {
            "test_description"
        }
    }

    impl Description for TestDescription {
        fn short(&self, _: &dyn World) -> String {
            "Test Event".into()
        }
        fn long(&self, _: &dyn World) -> String {
            "Test event to do something".into()
        }
    }

    #[derive(Debug, Clone)]
    struct TestEvent {
        id: Uuid,
        description: TestDescription,
    }
    impl Id for TestEvent {
        fn id(&self) -> &Uuid {
            &self.id
        }
        fn set_id(&mut self, id: Uuid) {
            self.id = id
        }
        fn roles(&self) -> Vec<&'static str> {
            vec![]
        }
    }
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
    impl Event for TestEvent {
        fn trigger(&mut self, _world: &mut dyn World) {}

        fn can_be_triggered(&self, _world: &dyn World) -> bool {
            true
        }

        fn translation_base(&self) -> String {
            "test_event".into()
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
    }

    #[derive(Debug, Default)]
    struct TestWorld {
        id: Uuid,
        lang: String,
        items: HashMap<String, Box<dyn Item>>,
        scenes: HashMap<String, Box<dyn Scene>>,
        characters: HashMap<String, Box<dyn Character>>,
    }

    impl Id for TestWorld {
        fn id(&self) -> &Uuid {
            &self.id
        }
        fn set_id(&mut self, id: Uuid) {
            self.id = id
        }
        fn roles(&self) -> Vec<&'static str> {
            vec![]
        }
    }

    impl Named for TestWorld {
        fn name(&self) -> &'static str {
            "test_world"
        }
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

        fn available_languages(&self) -> Vec<&str> {
            vec!["en-US"]
        }

        #[cfg(feature = "with_world_setup")]
        fn setup(&mut self) {}

        fn finished(&self) -> bool {
            true
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
