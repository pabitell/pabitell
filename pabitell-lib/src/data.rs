use crate::{events, Event, Id};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait EventData<'a, E>: From<E> + Into<E> + Serialize + Deserialize<'a>
where
    E: Event,
{
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PickData {
    id: Uuid,
    name: String,
    character: String,
    item: String,
}

impl Into<events::Pick> for PickData {
    fn into(self) -> events::Pick {
        let PickData {
            id,
            name,
            character,
            item,
        } = self;

        let mut event = events::Pick::new(name, character, item);
        event.set_id(id);
        event
    }
}

impl From<events::Pick> for PickData {
    fn from(pick: events::Pick) -> Self {
        Self {
            id: pick.id().clone(),
            name: pick.name().to_string(),
            character: pick.character().to_string(),
            item: pick.item().to_string(),
        }
    }
}

impl PickData {
    pub fn new(id: Uuid, name: String, character: String, item: String) -> Self {
        Self {
            id,
            name,
            character,
            item,
        }
    }
}
impl<'a> EventData<'a, events::Pick> for PickData {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GiveData {
    id: Uuid,
    name: String,
    from_character: String,
    to_character: String,
    item: String,
}

impl Into<events::Give> for GiveData {
    fn into(self) -> events::Give {
        let GiveData {
            id,
            name,
            from_character,
            to_character,
            item,
        } = self;

        let mut event = events::Give::new(name, from_character, to_character, item);
        event.set_id(id);
        event
    }
}

impl From<events::Give> for GiveData {
    fn from(give: events::Give) -> Self {
        Self {
            id: give.id().clone(),
            name: give.name().to_string(),
            from_character: give.from_character().to_string(),
            to_character: give.to_character().to_string(),
            item: give.item().to_string(),
        }
    }
}

impl GiveData {
    pub fn new(
        id: Uuid,
        name: String,
        from_character: String,
        to_character: String,
        item: String,
    ) -> Self {
        Self {
            id,
            name,
            from_character,
            to_character,
            item,
        }
    }
}
impl<'a> EventData<'a, events::Give> for GiveData {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UseItemData {
    id: Uuid,
    name: String,
    character: String,
    item: String,
}

impl Into<events::UseItem> for UseItemData {
    fn into(self) -> events::UseItem {
        let UseItemData {
            id,
            name,
            character,
            item,
        } = self;

        let mut event = events::UseItem::new(name, character, item);
        event.set_id(id);
        event
    }
}

impl From<events::UseItem> for UseItemData {
    fn from(use_item: events::UseItem) -> Self {
        Self {
            id: use_item.id().clone(),
            name: use_item.name().to_string(),
            character: use_item.character().to_string(),
            item: use_item.item().to_string(),
        }
    }
}

impl UseItemData {
    pub fn new(id: Uuid, name: String, character: String, item: String) -> Self {
        Self {
            id,
            name,
            character,
            item,
        }
    }
}
impl<'a> EventData<'a, events::UseItem> for UseItemData {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveData {
    id: Uuid,
    name: String,
    character: String,
    scene: String,
}

impl Into<events::Move> for MoveData {
    fn into(self) -> events::Move {
        let MoveData {
            id,
            name,
            character,
            scene,
        } = self;

        let mut event = events::Move::new(name, character, scene);
        event.set_id(id);
        event
    }
}

impl From<events::Move> for MoveData {
    fn from(move_event: events::Move) -> Self {
        Self {
            id: move_event.id().clone(),
            name: move_event.name().to_string(),
            character: move_event.character().to_string(),
            scene: move_event.scene().to_string(),
        }
    }
}

impl MoveData {
    pub fn new(id: Uuid, name: String, character: String, scene: String) -> Self {
        Self {
            id,
            name,
            character,
            scene,
        }
    }
}
impl<'a> EventData<'a, events::Move> for MoveData {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct VoidData {
    id: Uuid,
    name: String,
    character: String,
    item: Option<String>,
}

impl Into<events::Void> for VoidData {
    fn into(self) -> events::Void {
        let VoidData {
            id,
            name,
            character,
            item,
        } = self;

        let mut event = events::Void::new(name, character, item);
        event.set_id(id);
        event
    }
}

impl From<events::Void> for VoidData {
    fn from(pick: events::Void) -> Self {
        Self {
            id: pick.id().clone(),
            name: pick.name().to_string(),
            character: pick.character().to_string(),
            item: pick.item().clone(),
        }
    }
}

impl VoidData {
    pub fn new(id: Uuid, name: String, character: String, item: Option<String>) -> Self {
        Self {
            id,
            name,
            character,
            item,
        }
    }
}
impl<'a> EventData<'a, events::Void> for VoidData {}

#[cfg(test)]
pub mod test {
    use super::{EventData, GiveData, MoveData, PickData, UseItemData, VoidData};
    use crate::{events, Event};
    use uuid::Uuid;

    #[test]
    fn pick() {
        let data = PickData::new(
            Uuid::default(),
            "pick".to_string(),
            "character".to_string(),
            "item".to_string(),
        );
        let event: events::Pick = data.clone().into();
        let converted_data = PickData::from(event);
        assert_eq!(data, converted_data);
    }

    #[test]
    fn give() {
        let data = GiveData::new(
            Uuid::default(),
            "give".to_string(),
            "from_character".to_string(),
            "to_character".to_string(),
            "item".to_string(),
        );
        let event: events::Give = data.clone().into();
        let converted_data = GiveData::from(event);
        assert_eq!(data, converted_data);
    }

    #[test]
    fn use_item() {
        let data = UseItemData::new(
            Uuid::default(),
            "use_item".to_string(),
            "character".to_string(),
            "item".to_string(),
        );
        let event: events::UseItem = data.clone().into();
        let converted_data = UseItemData::from(event);
        assert_eq!(data, converted_data);
    }

    #[test]
    fn move_event() {
        let data = MoveData::new(
            Uuid::default(),
            "move".to_string(),
            "character".to_string(),
            "scene".to_string(),
        );
        let event: events::Move = data.clone().into();
        let converted_data = MoveData::from(event);
        assert_eq!(data, converted_data);
    }

    #[test]
    fn void() {
        let data = VoidData::new(
            Uuid::default(),
            "void".to_string(),
            "character".to_string(),
            Some("item".to_string()),
        );
        let event: events::Void = data.clone().into();
        let converted_data = VoidData::from(event);
        assert_eq!(data, converted_data);
    }
}
