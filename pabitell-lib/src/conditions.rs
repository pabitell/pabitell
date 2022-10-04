use crate::{ItemState, World};
use anyhow::{anyhow, Result};
use std::{collections::HashSet, fmt, ops, rc::Rc};

pub trait Check: fmt::Debug {
    fn check(&self, world: &dyn World) -> Result<bool>;
}

pub enum Condition {
    Check(Rc<dyn Check>),
    Not(Rc<Condition>),
    And(Rc<Condition>, Rc<Condition>),
    Or(Rc<Condition>, Rc<Condition>),
}

impl Condition {
    pub fn new(check: impl Check + 'static) -> Self {
        Self::Check(Rc::new(check))
    }
}

impl Default for Condition {
    fn default() -> Self {
        // By default always return true
        Self::Check(Rc::new(AlwaysCheck))
    }
}

impl Check for Condition {
    fn check(&self, world: &dyn World) -> Result<bool> {
        match self {
            Self::Check(check) => Ok(check.check(world)?),
            Self::Not(cond) => Ok(!cond.check(world)?),
            Self::And(cond1, cond2) => Ok(cond1.check(world)? && cond2.check(world)?),
            Self::Or(cond1, cond2) => Ok(cond1.check(world)? || cond2.check(world)?),
        }
    }
}

impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Check(check) => f.debug_struct("Check").field("check", check).finish(),
            Self::Not(cond) => f.debug_struct("Not").field("cond", cond).finish(),
            Self::And(cond1, cond2) => f
                .debug_struct("And")
                .field("cond1", cond1)
                .field("cond2", cond2)
                .finish(),
            Self::Or(cond1, cond2) => f
                .debug_struct("Or")
                .field("cond1", cond1)
                .field("cond2", cond2)
                .finish(),
        }
    }
}

impl ops::Not for Condition {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Not(Rc::new(self))
    }
}

impl ops::BitAnd for Condition {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::And(Rc::new(self), Rc::new(rhs))
    }
}

impl ops::BitOr for Condition {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(Rc::new(self), Rc::new(rhs))
    }
}

pub struct AlwaysCheck;

impl AlwaysCheck {
    pub fn cond() -> Condition {
        Self.into()
    }
}

impl fmt::Debug for AlwaysCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Always").finish()
    }
}

impl Check for AlwaysCheck {
    fn check(&self, _world: &dyn World) -> Result<bool> {
        Ok(true)
    }
}

impl From<AlwaysCheck> for Condition {
    fn from(check: AlwaysCheck) -> Self {
        Condition::Check(Rc::new(check))
    }
}

pub struct SameSceneCheck {
    characters: Vec<String>,
    items: Vec<String>,
}

impl SameSceneCheck {
    pub fn cond(characters: Vec<String>, items: Vec<String>) -> Condition {
        Self { characters, items }.into()
    }
}

impl fmt::Debug for SameSceneCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SameScene")
            .field("characters", &self.characters)
            .field("items", &self.items)
            .finish()
    }
}

impl Check for SameSceneCheck {
    fn check(&self, world: &dyn World) -> Result<bool> {
        let character_scenes = self
            .characters
            .iter()
            .filter_map(|e| world.characters().get(e.as_str())?.scene().clone())
            .collect::<Vec<_>>();
        let item_scenes = self
            .items
            .iter()
            .filter_map(|e| match world.items().get(e.as_str())?.state() {
                ItemState::InScene(scene) => Some(scene),
                _ => None,
            })
            .collect::<Vec<_>>();
        // All characters and items are in some scene
        if character_scenes.len() == self.characters.len() && item_scenes.len() == self.items.len()
        {
            if character_scenes
                .iter()
                .chain(item_scenes.into_iter())
                .collect::<HashSet<_>>()
                .len()
                < 2
            {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl From<SameSceneCheck> for Condition {
    fn from(check: SameSceneCheck) -> Self {
        Condition::new(check)
    }
}

pub struct HasItemCheck {
    character: String,
    item: String,
}

impl HasItemCheck {
    pub fn cond(character: String, item: String) -> Condition {
        Self { character, item }.into()
    }
}

impl fmt::Debug for HasItemCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HasItem")
            .field("character", &self.character)
            .field("item", &self.item)
            .finish()
    }
}

impl Check for HasItemCheck {
    fn check(&self, world: &dyn World) -> Result<bool> {
        if let Some(item) = world.items().get(&self.item) {
            match item.state() {
                ItemState::Owned(owner) => Ok(&self.character == owner),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}

impl From<HasItemCheck> for Condition {
    fn from(check: HasItemCheck) -> Self {
        Condition::new(check)
    }
}

pub struct CharacterInSceneCheck {
    character: String,
    scene: Option<String>,
}

impl CharacterInSceneCheck {
    pub fn cond(character: String, scene: Option<String>) -> Condition {
        Self { character, scene }.into()
    }
}

impl fmt::Debug for CharacterInSceneCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharacterInScene")
            .field("character", &self.character)
            .field("scene", &self.scene)
            .finish()
    }
}

impl Check for CharacterInSceneCheck {
    fn check(&self, world: &dyn World) -> Result<bool> {
        Ok(
            if let Some(character) = world.characters().get(&self.character) {
                &self.scene == character.scene()
            } else {
                false
            },
        )
    }
}

impl From<CharacterInSceneCheck> for Condition {
    fn from(check: CharacterInSceneCheck) -> Self {
        Condition::new(check)
    }
}

pub struct CanGiveCheck {
    from_character: String,
    to_character: String,
    item: String,
}

impl CanGiveCheck {
    pub fn cond(from_character: String, to_character: String, item: String) -> Condition {
        Self {
            from_character,
            to_character,
            item,
        }
        .into()
    }
}

impl fmt::Debug for CanGiveCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CanGive")
            .field("from_character", &self.from_character)
            .field("to_character", &self.to_character)
            .field("item", &self.item)
            .finish()
    }
}

impl Check for CanGiveCheck {
    fn check(&self, world: &dyn World) -> Result<bool> {
        (HasItemCheck::cond(self.from_character.clone(), self.item.clone())
            & SameSceneCheck::cond(
                vec![self.from_character.clone(), self.to_character.clone()],
                vec![],
            ))
        .check(world)
    }
}

impl From<CanGiveCheck> for Condition {
    fn from(check: CanGiveCheck) -> Self {
        Condition::new(check)
    }
}

pub struct AllItemsWithTagInStateCheck {
    tags: Vec<String>,
    state: ItemState,
}

impl AllItemsWithTagInStateCheck {
    pub fn cond(tags: Vec<String>, state: ItemState) -> Condition {
        Self { tags, state }.into()
    }
}

impl fmt::Debug for AllItemsWithTagInStateCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllItemsWithTagInState")
            .field("tags", &self.tags)
            .field("state", &self.state)
            .finish()
    }
}

impl Check for AllItemsWithTagInStateCheck {
    fn check(&self, world: &dyn World) -> Result<bool> {
        Ok(world
            .items()
            .values()
            .filter(|e| e.get_tags().iter().any(|t| self.tags.contains(t)))
            .all(|e| e.state() == &self.state))
    }
}

impl From<AllItemsWithTagInStateCheck> for Condition {
    fn from(check: AllItemsWithTagInStateCheck) -> Self {
        Condition::new(check)
    }
}

pub struct SceneDialogCheck {
    scene: String,
    dialog: usize,
}

impl SceneDialogCheck {
    pub fn cond(scene: String, dialog: usize) -> Condition {
        Self { scene, dialog }.into()
    }
}

impl fmt::Debug for SceneDialogCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SceneDialog")
            .field("scene", &self.scene)
            .field("dialog", &self.dialog)
            .finish()
    }
}

impl Check for SceneDialogCheck {
    fn check(&self, world: &dyn World) -> Result<bool> {
        Ok(world
            .scenes()
            .get(&self.scene)
            .ok_or_else(|| anyhow!("Scene {} not found", &self.scene))?
            .dialog()
            .ok_or_else(|| anyhow!("Scene {} doesn't have dialogs", &self.scene))?
            == self.dialog)
    }
}

impl From<SceneDialogCheck> for Condition {
    fn from(check: SceneDialogCheck) -> Self {
        Condition::new(check)
    }
}
