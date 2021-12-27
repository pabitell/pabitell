use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait EventData: Serialize + DeserializeOwned + PartialEq {
    fn initiator(&self) -> String;

    fn set_initiator(&mut self, initiator: String);
}

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
impl EventData for PickData {
    fn initiator(&self) -> String {
        self.character.clone()
    }
    fn set_initiator(&mut self, initiator: String) {
        self.character = initiator;
    }
}

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
impl EventData for GiveData {
    fn initiator(&self) -> String {
        self.to_character.clone()
    }
    fn set_initiator(&mut self, initiator: String) {
        self.to_character = initiator;
    }
}

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
impl EventData for UseItemData {
    fn initiator(&self) -> String {
        self.character.clone()
    }
    fn set_initiator(&mut self, initiator: String) {
        self.character = initiator;
    }
}

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
impl EventData for MoveData {
    fn initiator(&self) -> String {
        self.character.clone()
    }
    fn set_initiator(&mut self, initiator: String) {
        self.character = initiator;
    }
}

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
impl EventData for VoidData {
    fn initiator(&self) -> String {
        self.character.clone()
    }
    fn set_initiator(&mut self, initiator: String) {
        self.character = initiator;
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TalkData {
    pub name: String,
    pub character: String,
    pub scene: String,
    pub dialog: usize,
}

impl TalkData {
    pub fn new<SN, SC, SS>(name: SN, character: SC, scene: SS, dialog: usize) -> Self
    where
        SN: ToString,
        SS: ToString,
        SC: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            scene: scene.to_string(),
            dialog,
        }
    }
}
impl EventData for TalkData {
    fn initiator(&self) -> String {
        self.character.clone()
    }
    fn set_initiator(&mut self, initiator: String) {
        self.character = initiator;
    }
}
