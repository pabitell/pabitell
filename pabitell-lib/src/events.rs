use crate::{
    conditions::Condition,
    data::{self, EventData},
    updates::Change,
    AsAny, Event, GeoLocation, Tagged, World,
};
use std::{any::Any, fmt};

pub type Text = Option<Box<dyn Fn(&dyn Event, &dyn World) -> String>>;

#[derive(Default)]
pub struct Pick {
    name: String,
    data: data::PickData,
    tags: Vec<String>,
    world_updates: Vec<Box<dyn Change>>,
    condition: Condition,
}

impl fmt::Debug for Pick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Pick({})", self.name()))
            .field("character", &self.data.character)
            .field("item", &self.data.item)
            .finish()
    }
}

impl Tagged for Pick {
    fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl AsAny for Pick {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PartialEq<[u8]> for Pick {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other_data) = serde_json::from_slice::<data::PickData>(other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Pick {
    fn name(&self) -> &str {
        &self.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_updates(&mut self, updates: Vec<Box<dyn Change>>) {
        self.world_updates = updates;
    }

    fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }

    fn get_world_updates(&self) -> &[Box<dyn Change>] {
        &self.world_updates
    }

    fn get_condition(&self) -> &Condition {
        &self.condition
    }

    fn dump(&self) -> serde_json::Value {
        let mut res = serde_json::to_value(self.data.clone()).unwrap();
        res["name"] = serde_json::Value::String(self.name().to_string());
        res
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }

    fn items(&self) -> Vec<String> {
        vec![self.data.item.to_string()]
    }

    fn characters(&self) -> Vec<String> {
        vec![self.character().to_string()]
    }

    fn msg_base(&self, world: &dyn World) -> String {
        format!("{}-{}_{}", world.name(), self.character(), self.name(),)
    }
}

impl Pick {
    pub fn new<S>(name: S, data: data::PickData) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            data,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.data.character
    }

    pub fn item(&self) -> &str {
        &self.data.item
    }
}

#[derive(Default)]
pub struct Give {
    name: String,
    data: data::GiveData,
    tags: Vec<String>,
    world_updates: Vec<Box<dyn Change>>,
    condition: Condition,
}

impl fmt::Debug for Give {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Give({})", self.name()))
            .field("from_character", &self.data.from_character)
            .field("to_character", &self.data.to_character)
            .field("item", &self.data.item)
            .finish()
    }
}

impl Tagged for Give {
    fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl AsAny for Give {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PartialEq<[u8]> for Give {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other_data) = serde_json::from_slice::<data::GiveData>(other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Give {
    fn name(&self) -> &str {
        &self.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator);
    }

    fn set_world_updates(&mut self, updates: Vec<Box<dyn Change>>) {
        self.world_updates = updates;
    }

    fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }

    fn get_world_updates(&self) -> &[Box<dyn Change>] {
        &self.world_updates
    }

    fn get_condition(&self) -> &Condition {
        &self.condition
    }

    fn dump(&self) -> serde_json::Value {
        let mut res = serde_json::to_value(self.data.clone()).unwrap();
        res["name"] = serde_json::Value::String(self.name().to_string());
        res
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }

    fn items(&self) -> Vec<String> {
        vec![self.data.item.to_string()]
    }

    fn characters(&self) -> Vec<String> {
        vec![
            self.from_character().to_string(),
            self.to_character().to_string(),
        ]
    }

    fn msg_base(&self, world: &dyn World) -> String {
        format!(
            "{}-{}_give_{}_to_{}",
            world.name(),
            self.from_character(),
            self.item(),
            self.to_character(),
        )
    }
}

impl Give {
    pub fn new<S>(name: S, data: data::GiveData) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            data,
            ..Default::default()
        }
    }

    pub fn from_character(&self) -> &str {
        &self.data.from_character
    }

    pub fn to_character(&self) -> &str {
        &self.data.to_character
    }

    pub fn item(&self) -> &str {
        &self.data.item
    }
}

#[derive(Default)]
pub struct UseItem {
    name: String,
    data: data::UseItemData,
    tags: Vec<String>,
    world_updates: Vec<Box<dyn Change>>,
    condition: Condition,
}

impl fmt::Debug for UseItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("UseItem({})", self.name()))
            .field("character", &self.data.character)
            .field("item", &self.data.item)
            .finish()
    }
}

impl Tagged for UseItem {
    fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl AsAny for UseItem {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PartialEq<[u8]> for UseItem {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other_data) = serde_json::from_slice::<data::UseItemData>(other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for UseItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_updates(&mut self, updates: Vec<Box<dyn Change>>) {
        self.world_updates = updates;
    }

    fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }

    fn get_world_updates(&self) -> &[Box<dyn Change>] {
        &self.world_updates
    }

    fn get_condition(&self) -> &Condition {
        &self.condition
    }

    fn dump(&self) -> serde_json::Value {
        let mut res = serde_json::to_value(self.data.clone()).unwrap();
        res["name"] = serde_json::Value::String(self.name().to_string());
        res
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }

    fn items(&self) -> Vec<String> {
        vec![self.data.item.to_string()]
    }

    fn characters(&self) -> Vec<String> {
        vec![self.character().to_string()]
    }

    fn msg_base(&self, world: &dyn World) -> String {
        format!("{}-{}_{}", world.name(), self.character(), self.name(),)
    }
}

impl UseItem {
    pub fn new<S>(name: S, data: data::UseItemData) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            data,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.data.character
    }

    pub fn item(&self) -> &str {
        &self.data.item
    }
}

#[derive(Default)]
pub struct Move {
    name: String,
    data: data::MoveData,
    tags: Vec<String>,
    world_updates: Vec<Box<dyn Change>>,
    condition: Condition,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Move({})", self.name()))
            .field("character", &self.data.character)
            .field("to_scene", &self.data.scene)
            .finish()
    }
}

impl Tagged for Move {
    fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl AsAny for Move {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PartialEq<[u8]> for Move {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other_data) = serde_json::from_slice::<data::MoveData>(other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Move {
    fn name(&self) -> &str {
        &self.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_updates(&mut self, updates: Vec<Box<dyn Change>>) {
        self.world_updates = updates;
    }

    fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }

    fn get_world_updates(&self) -> &[Box<dyn Change>] {
        &self.world_updates
    }

    fn get_condition(&self) -> &Condition {
        &self.condition
    }

    fn dump(&self) -> serde_json::Value {
        let mut res = serde_json::to_value(self.data.clone()).unwrap();
        res["name"] = serde_json::Value::String(self.name().to_string());
        res
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }

    fn items(&self) -> Vec<String> {
        vec![]
    }

    fn characters(&self) -> Vec<String> {
        vec![self.character().to_string()]
    }

    fn msg_base(&self, world: &dyn World) -> String {
        format!("{}-{}_{}", world.name(), self.character(), self.name(),)
    }

    fn geo_location(&self, world: &dyn World) -> Option<(String, Option<String>, GeoLocation)> {
        let scene = world.scenes().get(&self.data.scene).unwrap();
        scene.geo_location().map(|loc| {
            (
                self.character().to_string(),
                Some(scene.name().to_string()),
                loc,
            )
        })
    }
}

impl Move {
    pub fn new<S>(name: S, data: data::MoveData) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            data,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.data.character
    }

    pub fn scene(&self) -> &str {
        &self.data.scene
    }
}

#[derive(Default)]
pub struct Void {
    name: String,
    data: data::VoidData,
    tags: Vec<String>,
    world_updates: Vec<Box<dyn Change>>,
    condition: Condition,
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Void({})", self.name()))
            .field("character", &self.data.character)
            .field("item", &self.data.item)
            .finish()
    }
}

impl Tagged for Void {
    fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl AsAny for Void {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PartialEq<[u8]> for Void {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other_data) = serde_json::from_slice::<data::VoidData>(other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Void {
    fn name(&self) -> &str {
        &self.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_updates(&mut self, updates: Vec<Box<dyn Change>>) {
        self.world_updates = updates;
    }

    fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }

    fn get_world_updates(&self) -> &[Box<dyn Change>] {
        &self.world_updates
    }

    fn get_condition(&self) -> &Condition {
        &self.condition
    }

    fn dump(&self) -> serde_json::Value {
        let mut res = serde_json::to_value(self.data.clone()).unwrap();
        res["name"] = serde_json::Value::String(self.name().to_string());
        res
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }

    fn items(&self) -> Vec<String> {
        if let Some(item) = self.data.item.as_ref() {
            vec![item.to_string()]
        } else {
            vec![]
        }
    }

    fn characters(&self) -> Vec<String> {
        vec![self.character().to_string()]
    }

    fn msg_base(&self, world: &dyn World) -> String {
        format!(
            "{}-{}_{}_{}",
            world.name(),
            self.character(),
            self.name(),
            self.item().clone().unwrap_or_default(),
        )
    }
}

impl Void {
    pub fn new<S>(name: S, data: data::VoidData) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            data,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.data.character
    }

    pub fn item(&self) -> &Option<String> {
        &self.data.item
    }
}

#[derive(Default)]
pub struct Talk {
    name: String,
    data: data::TalkData,
    tags: Vec<String>,
    world_updates: Vec<Box<dyn Change>>,
    condition: Condition,
}

impl fmt::Debug for Talk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Talk({})", self.name()))
            .field("character", &self.data.character)
            .field("item", &self.data.scene)
            .field("dialog", &self.data.dialog)
            .finish()
    }
}

impl Tagged for Talk {
    fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl AsAny for Talk {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PartialEq<[u8]> for Talk {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other_data) = serde_json::from_slice::<data::TalkData>(other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Talk {
    fn name(&self) -> &str {
        &self.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_updates(&mut self, updates: Vec<Box<dyn Change>>) {
        self.world_updates = updates;
    }

    fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }

    fn get_world_updates(&self) -> &[Box<dyn Change>] {
        &self.world_updates
    }

    fn get_condition(&self) -> &Condition {
        &self.condition
    }

    fn dump(&self) -> serde_json::Value {
        let mut res = serde_json::to_value(self.data.clone()).unwrap();
        res["name"] = serde_json::Value::String(self.name().to_string());
        res
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }

    fn items(&self) -> Vec<String> {
        vec![]
    }

    fn characters(&self) -> Vec<String> {
        vec![self.character().to_string()]
    }

    fn msg_base(&self, world: &dyn World) -> String {
        format!(
            "{}-{}_{}_says-{}",
            world.name(),
            self.scene(),
            self.character(),
            self.dialog()
        )
    }
}

impl Talk {
    pub fn new<S>(name: S, data: data::TalkData) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            data,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.data.character
    }

    pub fn scene(&self) -> &str {
        &self.data.scene
    }

    pub fn dialog(&self) -> usize {
        self.data.dialog
    }
}

#[cfg(test)]
pub mod test {
    use super::{Give, Move, Pick, Talk, UseItem, Void};
    use crate::{data, Event};

    #[test]
    fn kinds() {
        let pick = Pick::new("pick", data::PickData::new("character", "item"));
        assert_eq!(pick.kind(), "Pick");

        let give = Give::new(
            "give",
            data::GiveData::new("from_character", "to_character", "item"),
        );
        assert_eq!(give.kind(), "Give");

        let move_event = Move::new("move", data::MoveData::new("character", "to_scene"));
        assert_eq!(move_event.kind(), "Move");

        let use_item = UseItem::new("use_item", data::UseItemData::new("character", "item"));
        assert_eq!(use_item.kind(), "UseItem");

        let void = Void::new(
            "void",
            data::VoidData::new("character", None as Option<String>),
        );
        assert_eq!(void.kind(), "Void");

        let talk = Talk::new("talk", data::TalkData::new("character", "scene", 0));
        assert_eq!(talk.kind(), "Talk");
    }
}
