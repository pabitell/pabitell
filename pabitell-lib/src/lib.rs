use anyhow::Result;
use std::fmt;
use uuid;

pub trait Text {
    fn plain(&self) -> &str;
}

pub trait Id {
    fn id(&self) -> &uuid::Uuid;
}

pub trait Item: Id + fmt::Debug {
    fn quantity(&self) -> Option<usize>;
}

pub trait Character: Id + fmt::Debug {
    fn items(&self) -> &[usize];
}

pub trait Location: Id + fmt::Debug {
    fn characters(&self) -> &[usize];
    fn items(&self) -> &[usize];
}

pub trait StoryBuilder<S>
where
    S: Story,
{
    fn id(self, id: uuid::Uuid) -> Self;
    fn intro(self, text: Box<dyn Text>) -> Self;
    fn outro(self, text: Box<dyn Text>) -> Self;
    fn description(self, text: Box<dyn Text>) -> Self;
    fn character(self, character: Box<dyn Character>) -> Self;
    fn item(self, item: Box<dyn Item>) -> Self;
    fn location(self, item: Box<dyn Location>) -> Self;
    fn ready(&self) -> bool;
    fn build(self) -> Result<S>;
}

pub trait Story: Id {
    fn description(&self) -> &Box<dyn Text>;
    fn locations(&self) -> &[Box<dyn Location>];
    fn characters(&self) -> &[Box<dyn Character>];
    fn items(&self) -> &[Box<dyn Item>];
}

#[cfg(test)]
pub mod test {
    use super::{Character, Id, Item, Location, Story, StoryBuilder, Text};
    use anyhow::{anyhow, Result};
    use uuid;

    #[derive(Debug, Default)]
    struct TestCharacter {
        id: uuid::Uuid,
        items: Vec<usize>,
    }

    impl Id for TestCharacter {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl Character for TestCharacter {
        fn items(&self) -> &[usize] {
            &self.items
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

    impl Item for TestItem {
        fn quantity(&self) -> Option<usize> {
            Some(1)
        }
    }

    #[derive(Debug, Default)]
    struct TestLocation {
        id: uuid::Uuid,
        characters: Vec<usize>,
        items: Vec<usize>,
    }

    impl Id for TestLocation {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl Location for TestLocation {
        fn characters(&self) -> &[usize] {
            &self.characters
        }

        fn items(&self) -> &[usize] {
            &self.items
        }
    }

    struct TestText {
        text: String,
    }
    impl Text for TestText {
        fn plain(&self) -> &str {
            &self.text
        }
    }

    struct TestStory {
        id: uuid::Uuid,
        description: Box<dyn Text>,
        items: Vec<Box<dyn Item>>,
        locations: Vec<Box<dyn Location>>,
        characters: Vec<Box<dyn Character>>,
    }

    impl Id for TestStory {
        fn id(&self) -> &uuid::Uuid {
            &self.id
        }
    }

    impl Story for TestStory {
        fn description(&self) -> &Box<dyn Text> {
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
        intro: Option<Box<dyn Text>>,
        outro: Option<Box<dyn Text>>,
        description: Option<Box<dyn Text>>,
        characters: Vec<Box<dyn Character>>,
        items: Vec<Box<dyn Item>>,
        locations: Vec<Box<dyn Location>>,
    }

    impl StoryBuilder<TestStory> for TestStoryBuilder {
        fn id(mut self, id: uuid::Uuid) -> Self {
            self.id = Some(id);
            self
        }

        fn intro(mut self, text: Box<dyn Text>) -> Self {
            self.intro = Some(text);
            self
        }

        fn outro(mut self, text: Box<dyn Text>) -> Self {
            self.outro = Some(text);
            self
        }

        fn description(mut self, text: Box<dyn Text>) -> Self {
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
            .intro(Box::new(TestText {
                text: "Some intro".into(),
            }))
            .outro(Box::new(TestText {
                text: "Some outro".into(),
            }))
            .description(Box::new(TestText {
                text: "Some description".into(),
            }))
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
