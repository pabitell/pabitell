use anyhow::Result;
use std::fmt;
use uuid;

pub trait Id {
    fn id(&self) -> &uuid::Uuid;
}

pub trait Description {
    fn detail(&self, world: &Box<dyn World>) -> &str;
    fn short(&self, world: &Box<dyn World>) -> &str;
}

pub trait Item: Id + Description + fmt::Debug {
    fn quantity(&self) -> Option<usize>;
}

pub trait Character: Id + Description + fmt::Debug {
    fn items(&self) -> &[usize];
    fn location(&self) -> Option<usize>;
}

pub trait Location: Id + Description + fmt::Debug {
    fn items(&self) -> &[usize];
}

pub trait Event: Id + fmt::Debug {
    fn characters(&self) -> Vec<usize> {
        // Characters related to event
        vec![]
    }
    fn can_be_triggered(&self, world: &Box<dyn World>) -> bool;
    fn trigger(self, world: &mut Box<dyn World>) -> Option<Box<dyn Description>>;
}

pub trait WorldBuilder<S>
where
    S: World,
{
    fn id(self, id: uuid::Uuid) -> Self;
    fn description(self, text: Box<dyn Description>) -> Self;
    fn character(self, character: Box<dyn Character>) -> Self;
    fn item(self, item: Box<dyn Item>) -> Self;
    fn location(self, item: Box<dyn Location>) -> Self;
    fn ready(&self) -> bool;
    fn build(self) -> Result<S>;
}

pub trait World: Id {
    fn description(&self) -> &Box<dyn Description>;
    fn locations(&self) -> &[Box<dyn Location>];
    fn characters(&self) -> &[Box<dyn Character>];
    fn items(&self) -> &[Box<dyn Item>];
}

#[cfg(test)]
pub mod test {
    use super::{Character, Description, Event, Id, Item, Location, World, WorldBuilder};
    use anyhow::{anyhow, Result};
    use uuid;

    #[derive(Debug, Default)]
    struct TestCharacter {
        id: uuid::Uuid,
        items: Vec<usize>,
        location: Option<usize>,
    }

    impl Id for TestCharacter {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl Description for TestCharacter {
        fn short(&self, _: &Box<dyn World>) -> &str {
            "Test Character"
        }
        fn detail(&self, _: &Box<dyn World>) -> &str {
            "Character description and perhaps items which it is carying"
        }
    }

    impl Character for TestCharacter {
        fn items(&self) -> &[usize] {
            &self.items
        }
        fn location(&self) -> Option<usize> {
            self.location
        }
    }

    #[derive(Debug, Default)]
    struct TestItem {
        id: uuid::Uuid,
        quantity: Option<usize>,
    }

    impl Id for TestItem {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl Description for TestItem {
        fn short(&self, _: &Box<dyn World>) -> &str {
            "Test Location"
        }
        fn detail(&self, _: &Box<dyn World>) -> &str {
            "Test location detailed description"
        }
    }

    impl Item for TestItem {
        fn quantity(&self) -> Option<usize> {
            Some(1)
        }
    }

    #[derive(Debug, Default)]
    struct TestLocation {
        id: uuid::Uuid,
        items: Vec<usize>,
    }

    impl Id for TestLocation {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl Description for TestLocation {
        fn short(&self, _: &Box<dyn World>) -> &str {
            "Test Location"
        }
        fn detail(&self, _: &Box<dyn World>) -> &str {
            "Test location detailed description"
        }
    }

    impl Location for TestLocation {
        fn items(&self) -> &[usize] {
            &self.items
        }
    }

    #[derive(Clone, Debug)]
    struct TestDescription;
    impl Description for TestDescription {
        fn short(&self, _: &Box<dyn World>) -> &str {
            "Test Event"
        }
        fn detail(&self, _: &Box<dyn World>) -> &str {
            "Test event to do something"
        }
    }

    #[derive(Debug, Clone)]
    struct TestEvent {
        id: uuid::Uuid,
        description: TestDescription,
        success: TestDescription,
        failure: TestDescription,
    }
    impl Id for TestEvent {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }
    impl Event for TestEvent {
        fn trigger(self, world: &mut Box<dyn World>) -> Option<Box<dyn Description>> {
            if self.can_be_triggered(world) {
                return Some(Box::new(TestDescription));
            } else {
                None
            }
        }

        fn can_be_triggered(&self, world: &Box<dyn World>) -> bool {
            if let Some(location) = world.characters()[0].location() {
                if location == 0 {
                    return true;
                }
            }

            false
        }
    }

    struct TestStory {
        id: uuid::Uuid,
        description: Box<dyn Description>,
        items: Vec<Box<dyn Item>>,
        locations: Vec<Box<dyn Location>>,
        characters: Vec<Box<dyn Character>>,
    }

    impl Id for TestStory {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl World for TestStory {
        fn description(&self) -> &Box<dyn Description> {
            &self.description
        }

        fn items(&self) -> &[Box<dyn Item>] {
            &self.items
        }

        fn locations(&self) -> &[Box<dyn Location>] {
            &self.locations
        }

        fn characters(&self) -> &[Box<dyn Character>] {
            &self.characters
        }
    }

    #[derive(Default)]
    struct TestStoryBuilder {
        id: Option<uuid::Uuid>,
        description: Option<Box<dyn Description>>,
        characters: Vec<Box<dyn Character>>,
        items: Vec<Box<dyn Item>>,
        locations: Vec<Box<dyn Location>>,
    }

    impl WorldBuilder<TestStory> for TestStoryBuilder {
        fn id(mut self, id: uuid::Uuid) -> Self {
            self.id = Some(id);
            self
        }

        fn description(mut self, text: Box<dyn Description>) -> Self {
            self.description = Some(text);
            self
        }

        fn character(mut self, character: Box<dyn Character>) -> Self {
            self.characters.push(character);
            self
        }

        fn item(mut self, item: Box<dyn Item>) -> Self {
            self.items.push(item);
            self
        }

        fn location(mut self, item: Box<dyn Location>) -> Self {
            self.locations.push(item);
            self
        }

        fn ready(&self) -> bool {
            return self.id.is_some() && !self.characters.is_empty();
        }

        fn build(self) -> Result<TestStory> {
            Ok(TestStory {
                id: self.id.ok_or_else(|| anyhow!("Missing id"))?,
                description: self
                    .description
                    .ok_or_else(|| anyhow!("Missing description"))?,
                characters: self.characters,
                items: self.items,
                locations: self.locations,
            })
        }
    }

    #[test]
    fn linear() {
        let story = TestStoryBuilder::default()
            .id(uuid::Uuid::new_v4())
            .description(Box::new(TestDescription))
            .character(Box::new(TestCharacter::default()))
            .character(Box::new(TestCharacter::default()))
            .item(Box::new(TestItem::default()))
            .item(Box::new(TestItem::default()))
            .item(Box::new(TestItem::default()))
            .item(Box::new(TestItem::default()))
            .location(Box::new(TestLocation::default()))
            .location(Box::new(TestLocation::default()))
            .location(Box::new(TestLocation::default()));

        assert_eq!(story.ready(), true);
        assert!(story.build().is_ok());
    }
}
