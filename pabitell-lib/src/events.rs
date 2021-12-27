use crate::{
    data::{self, EventData},
    AsAny, Event, Tagged, World,
};
use std::{any::Any, fmt};

#[derive(Default)]
pub struct Pick {
    data: data::PickData,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
}

impl fmt::Debug for Pick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("({})", self.name()))
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
        if let Ok(other_data) = serde_json::from_slice::<data::PickData>(&other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Pick {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_update(&mut self, update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>) {
        self.world_update = update;
    }

    fn set_condition(&mut self, condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>) {
        self.condition = condition;
    }

    fn set_make_action_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_action_text = text;
    }

    fn set_make_success_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_success_text = text;
    }
    fn set_make_fail_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_fail_text = text;
    }

    fn get_world_update(&self) -> &Option<Box<dyn Fn(&dyn Any, &mut dyn World)>> {
        &self.world_update
    }

    fn get_condition(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>> {
        &self.condition
    }

    fn get_make_action_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_action_text
    }

    fn get_make_success_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_success_text
    }

    fn get_make_fail_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_fail_text
    }

    fn dump(&self) -> serde_json::Value {
        serde_json::to_value(self.data.clone()).unwrap()
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }
}

impl Pick {
    pub fn new(data: data::PickData) -> Self {
        Self {
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
    data: data::GiveData,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
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
        if let Ok(other_data) = serde_json::from_slice::<data::GiveData>(&other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Give {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator);
    }

    fn set_world_update(&mut self, update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>) {
        self.world_update = update;
    }

    fn set_condition(&mut self, condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>) {
        self.condition = condition;
    }

    fn set_make_action_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_action_text = text;
    }

    fn set_make_success_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_success_text = text;
    }

    fn set_make_fail_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_fail_text = text;
    }

    fn get_world_update(&self) -> &Option<Box<dyn Fn(&dyn Any, &mut dyn World)>> {
        &self.world_update
    }

    fn get_condition(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>> {
        &self.condition
    }

    fn get_make_action_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_action_text
    }

    fn get_make_success_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_success_text
    }

    fn get_make_fail_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_fail_text
    }

    fn dump(&self) -> serde_json::Value {
        serde_json::to_value(self.data.clone()).unwrap()
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }
}

impl Give {
    pub fn new(data: data::GiveData) -> Self {
        Self {
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
    data: data::UseItemData,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
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
        if let Ok(other_data) = serde_json::from_slice::<data::UseItemData>(&other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for UseItem {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_update(&mut self, update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>) {
        self.world_update = update;
    }

    fn set_condition(&mut self, condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>) {
        self.condition = condition;
    }

    fn set_make_action_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_action_text = text;
    }

    fn set_make_success_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_success_text = text;
    }
    fn set_make_fail_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_fail_text = text;
    }

    fn get_world_update(&self) -> &Option<Box<dyn Fn(&dyn Any, &mut dyn World)>> {
        &self.world_update
    }

    fn get_condition(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>> {
        &self.condition
    }

    fn get_make_action_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_action_text
    }

    fn get_make_success_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_success_text
    }

    fn get_make_fail_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_fail_text
    }

    fn dump(&self) -> serde_json::Value {
        serde_json::to_value(self.data.clone()).unwrap()
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }
}

impl UseItem {
    pub fn new(data: data::UseItemData) -> Self {
        Self {
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
    data: data::MoveData,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
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
        if let Ok(other_data) = serde_json::from_slice::<data::MoveData>(&other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Move {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_update(&mut self, update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>) {
        self.world_update = update;
    }

    fn set_condition(&mut self, condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>) {
        self.condition = condition;
    }

    fn set_make_action_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_action_text = text;
    }

    fn set_make_success_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_success_text = text;
    }
    fn set_make_fail_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_fail_text = text;
    }

    fn get_world_update(&self) -> &Option<Box<dyn Fn(&dyn Any, &mut dyn World)>> {
        &self.world_update
    }

    fn get_condition(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>> {
        &self.condition
    }

    fn get_make_action_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_action_text
    }

    fn get_make_success_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_success_text
    }

    fn get_make_fail_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_fail_text
    }

    fn dump(&self) -> serde_json::Value {
        serde_json::to_value(self.data.clone()).unwrap()
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }
}

impl Move {
    pub fn new(data: data::MoveData) -> Self {
        Self {
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
    data: data::VoidData,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
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
        if let Ok(other_data) = serde_json::from_slice::<data::VoidData>(&other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Void {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_update(&mut self, update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>) {
        self.world_update = update;
    }

    fn set_condition(&mut self, condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>) {
        self.condition = condition;
    }

    fn set_make_action_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_action_text = text;
    }

    fn set_make_success_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_success_text = text;
    }
    fn set_make_fail_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_fail_text = text;
    }

    fn get_world_update(&self) -> &Option<Box<dyn Fn(&dyn Any, &mut dyn World)>> {
        &self.world_update
    }

    fn get_condition(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>> {
        &self.condition
    }

    fn get_make_action_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_action_text
    }

    fn get_make_success_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_success_text
    }

    fn get_make_fail_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_fail_text
    }

    fn dump(&self) -> serde_json::Value {
        serde_json::to_value(self.data.clone()).unwrap()
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }
}

impl Void {
    pub fn new(data: data::VoidData) -> Self {
        Self {
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
    data: data::TalkData,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
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
        if let Ok(other_data) = serde_json::from_slice::<data::TalkData>(&other) {
            self.data == other_data
        } else {
            false
        }
    }
}

impl Event for Talk {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn initiator(&self) -> String {
        self.data.initiator()
    }

    fn set_initiator(&mut self, initiator: String) {
        self.data.set_initiator(initiator)
    }

    fn set_world_update(&mut self, update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>) {
        self.world_update = update;
    }

    fn set_condition(&mut self, condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>) {
        self.condition = condition;
    }

    fn set_make_action_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_action_text = text;
    }

    fn set_make_success_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_success_text = text;
    }
    fn set_make_fail_text(&mut self, text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>) {
        self.make_fail_text = text;
    }

    fn get_world_update(&self) -> &Option<Box<dyn Fn(&dyn Any, &mut dyn World)>> {
        &self.world_update
    }

    fn get_condition(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>> {
        &self.condition
    }

    fn get_make_action_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_action_text
    }

    fn get_make_success_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_success_text
    }

    fn get_make_fail_text(&self) -> &Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>> {
        &self.make_fail_text
    }

    fn dump(&self) -> serde_json::Value {
        serde_json::to_value(self.data.clone()).unwrap()
    }

    fn matches(&self, value: &serde_json::Value) -> bool {
        &self.dump() == value
    }
}

impl Talk {
    pub fn new(data: data::TalkData) -> Self {
        Self {
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
    use super::{Give, Move, Pick, UseItem, Void};
    use crate::{data, Event};

    #[test]
    fn kinds() {
        let pick = Pick::new(data::PickData::new("pick", "character", "item"));
        assert_eq!(pick.kind(), "Pick");

        let give = Give::new(data::GiveData::new(
            "give",
            "from_character",
            "to_character",
            "item",
        ));
        assert_eq!(give.kind(), "Give");

        let move_event = Move::new(data::MoveData::new("move", "character", "to_scene"));
        assert_eq!(move_event.kind(), "Move");

        let use_item = UseItem::new(data::UseItemData::new("use_item", "character", "item"));
        assert_eq!(use_item.kind(), "UseItem");

        let void = Void::new(data::VoidData::new(
            "void",
            "character",
            None as Option<String>,
        ));
        assert_eq!(void.kind(), "Void");
    }
}
