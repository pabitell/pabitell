use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait EventData: Serialize + DeserializeOwned + PartialEq {
    fn initiator(&self) -> String;

    fn set_initiator(&mut self, initiator: String);
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PickData {
    pub character: String,
    pub item: String,
}

impl PickData {
    pub fn new<SC, SI>(character: SC, item: SI) -> Self
    where
        SC: ToString,
        SI: ToString,
    {
        Self {
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
    pub from_character: String,
    pub to_character: String,
    pub item: String,
}

impl GiveData {
    pub fn new<SFC, STC, SI>(from_character: SFC, to_character: STC, item: SI) -> Self
    where
        SFC: ToString,
        STC: ToString,
        SI: ToString,
    {
        Self {
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
    pub character: String,
    pub item: String,
}

impl UseItemData {
    pub fn new<SC, SI>(character: SC, item: SI) -> Self
    where
        SC: ToString,
        SI: ToString,
    {
        Self {
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
    pub character: String,
    pub scene: String,
}

impl MoveData {
    pub fn new<SC, SS>(character: SC, scene: SS) -> Self
    where
        SC: ToString,
        SS: ToString,
    {
        Self {
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
    pub character: String,
    pub item: Option<String>,
}

impl VoidData {
    pub fn new<SC, SI>(character: SC, item: Option<SI>) -> Self
    where
        SI: ToString,
        SC: ToString,
    {
        Self {
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
    pub character: String,
    pub scene: String,
    pub dialog: usize,
}

impl TalkData {
    pub fn new<SC, SS>(character: SC, scene: SS, dialog: usize) -> Self
    where
        SS: ToString,
        SC: ToString,
    {
        Self {
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
