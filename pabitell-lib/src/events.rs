use crate::{AsAny, Event, Id, Tagged, World};
use std::{any::Any, fmt};
use uuid::Uuid;

#[derive(Default)]
pub struct Pick {
    id: Uuid,
    name: String,
    character: String,
    item: String,
    tags: Vec<String>,
    world_update: Option<Box<dyn Fn(&dyn Any, &mut dyn World)>>,
    condition: Option<Box<dyn Fn(&dyn Any, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&dyn Any, &dyn World) -> String>>,
}

impl fmt::Debug for Pick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Pick({})", self.name()))
            .field("character", &self.character)
            .field("item", &self.item)
            .finish()
    }
}

impl Id for Pick {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

impl Event for Pick {
    fn name(&self) -> &str {
        &self.name
    }

    fn translation_base(&self) -> String {
        format!("{}_{}_{}", self.character, self.name, self.item)
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
}

impl Pick {
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
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }

    pub fn item(&self) -> &str {
        &self.item
    }
}

#[derive(Default)]
pub struct Give {
    id: Uuid,
    name: String,
    from_character: String,
    to_character: String,
    item: String,
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
            .field("from_character", &self.from_character)
            .field("to_character", &self.to_character)
            .field("item", &self.item)
            .finish()
    }
}

impl Id for Give {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

impl Event for Give {
    fn name(&self) -> &str {
        &self.name
    }

    fn translation_base(&self) -> String {
        format!(
            "{}_{}_{}_to_{}",
            self.from_character, self.name, self.item, self.to_character
        )
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
}

impl Give {
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
            ..Default::default()
        }
    }

    pub fn from_character(&self) -> &str {
        &self.from_character
    }

    pub fn to_character(&self) -> &str {
        &self.to_character
    }

    pub fn item(&self) -> &str {
        &self.item
    }
}

#[derive(Default)]
pub struct UseItem {
    id: Uuid,
    name: String,
    character: String,
    item: String,
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
            .field("character", &self.character)
            .field("item", &self.item)
            .finish()
    }
}

impl Id for UseItem {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

impl Event for UseItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn translation_base(&self) -> String {
        format!("{}_{}_{}", self.character, self.name, self.item)
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
}

impl UseItem {
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
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }

    pub fn item(&self) -> &str {
        &self.item
    }
}

#[derive(Default)]
pub struct Move {
    id: Uuid,
    name: String,
    character: String,
    scene: String,
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
            .field("character", &self.character)
            .field("to_scene", &self.scene)
            .finish()
    }
}

impl Id for Move {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

impl Event for Move {
    fn name(&self) -> &str {
        &self.name
    }

    fn translation_base(&self) -> String {
        format!("{}_{}_to_{}", self.character, self.name, self.scene)
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
}

impl Move {
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
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }

    pub fn scene(&self) -> &str {
        &self.scene
    }
}

#[derive(Default)]
pub struct Void {
    id: Uuid,
    name: String,
    character: String,
    item: Option<String>,
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
            .field("character", &self.character)
            .field("item", &self.item)
            .finish()
    }
}

impl Id for Void {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

impl Event for Void {
    fn name(&self) -> &str {
        &self.name
    }

    fn translation_base(&self) -> String {
        if let Some(item) = self.item.as_ref() {
            format!("{}_{}_{}", self.character, self.name, item)
        } else {
            format!("{}_{}", self.character, self.name)
        }
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
}

impl Void {
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
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }

    pub fn item(&self) -> &Option<String> {
        &self.item
    }
}

#[cfg(test)]
pub mod test {
    use super::{Give, Move, Pick, UseItem, Void};
    use crate::Event;

    #[test]
    fn kinds() {
        let pick = Pick::new("pick", "character", "item");
        assert_eq!(pick.kind(), "Pick");

        let give = Give::new("give", "from_character", "to_character", "item");
        assert_eq!(give.kind(), "Give");

        let move_event = Move::new("move", "character", "to_scene");
        assert_eq!(move_event.kind(), "Move");

        let use_item = UseItem::new("use_item", "character", "item");
        assert_eq!(use_item.kind(), "UseItem");

        let void = Void::new("void", "character", None as Option<String>);
        assert_eq!(void.kind(), "Void");
    }
}
