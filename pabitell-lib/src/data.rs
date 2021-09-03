use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait EventData<'a>: Serialize + Deserialize<'a> {}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PickData {
    pub name: String,
    pub character: String,
    pub item: String,
}

impl PickData {
    pub fn new<SN, SC, SI>(name: SN, character: SC, item: SI) -> Self
    where
        SN: ToString,
        SC: ToString,
        SI: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            item: item.to_string(),
        }
    }
}
impl<'a> EventData<'a> for PickData {}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GiveData {
    pub name: String,
    pub from_character: String,
    pub to_character: String,
    pub item: String,
}

impl GiveData {
    pub fn new<SN, SFC, STC, SI>(name: SN, from_character: SFC, to_character: STC, item: SI) -> Self
    where
        SN: ToString,
        SFC: ToString,
        STC: ToString,
        SI: ToString,
    {
        Self {
            name: name.to_string(),
            from_character: from_character.to_string(),
            to_character: to_character.to_string(),
            item: item.to_string(),
        }
    }
}
impl<'a> EventData<'a> for GiveData {}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UseItemData {
    pub name: String,
    pub character: String,
    pub item: String,
}

impl UseItemData {
    pub fn new<SN, SC, SI>(name: SN, character: SC, item: SI) -> Self
    where
        SN: ToString,
        SC: ToString,
        SI: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            item: item.to_string(),
        }
    }
}
impl<'a> EventData<'a> for UseItemData {}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveData {
    pub name: String,
    pub character: String,
    pub scene: String,
}

impl MoveData {
    pub fn new<SN, SC, SS>(name: SN, character: SC, scene: SS) -> Self
    where
        SN: ToString,
        SC: ToString,
        SS: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            scene: scene.to_string(),
        }
    }
}
impl<'a> EventData<'a> for MoveData {}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct VoidData {
    pub name: String,
    pub character: String,
    pub item: Option<String>,
}

impl VoidData {
    pub fn new<SN, SC, SI>(name: SN, character: SC, item: Option<SI>) -> Self
    where
        SN: ToString,
        SI: ToString,
        SC: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            item: item.map(|e| e.to_string()),
        }
    }
}
impl<'a> EventData<'a> for VoidData {}
