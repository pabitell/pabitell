use crate::{AsAny, Event, Id, ItemState, World};
use std::{any::Any, fmt};
use uuid::Uuid;

#[derive(Default)]
pub struct Pick {
    id: Uuid,
    name: String,
    character: String,
    item: String,
    consume: bool,
    roles: Vec<&'static str>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
}

impl fmt::Debug for Pick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Pick({})", self.name()))
            .field("character", &self.character)
            .field("item", &self.item)
            .field("consume", &self.consume)
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

    fn roles(&self) -> Vec<&'static str> {
        self.roles.clone()
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

    fn can_be_triggered(&self, world: &dyn World) -> bool {
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }

        if let Some(scene) = world.characters().get(&self.character).unwrap().scene() {
            if &ItemState::InScene(scene.to_string())
                == world.items().get(&self.item).unwrap().state()
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn trigger(&mut self, world: &mut dyn World) {
        if let Some(world_update) = self.world_update.as_ref() {
            (world_update)(&self, world)
        }

        world
            .items_mut()
            .get_mut(&self.item)
            .unwrap()
            .set_state(if self.consume {
                ItemState::Unassigned
            } else {
                ItemState::Owned(self.character.to_string())
            });
    }

    fn translation_base(&self) -> String {
        format!("{}_{}_{}", self.character, self.name, self.item)
    }

    fn action_text(&self, world: &dyn World) -> String {
        self.make_action_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn success_text(&self, world: &dyn World) -> String {
        self.make_success_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn fail_text(&self, world: &dyn World) -> String {
        self.make_fail_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }
}

impl Pick {
    pub fn new<SN, SC, SI>(
        name: SN,
        character: SC,
        item: SI,
        consume: bool,
        roles: Vec<&'static str>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self
    where
        SN: ToString,
        SC: ToString,
        SI: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            item: item.to_string(),
            consume,
            roles,
            custom_condition,
            world_update,
            make_action_text,
            make_success_text,
            make_fail_text,
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
    consume: bool,
    roles: Vec<&'static str>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
}

impl fmt::Debug for Give {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Give({})", self.name()))
            .field("from_character", &self.from_character)
            .field("to_character", &self.to_character)
            .field("item", &self.item)
            .field("consume", &self.consume)
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

    fn roles(&self) -> Vec<&'static str> {
        self.roles.clone()
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

    fn can_be_triggered(&self, world: &dyn World) -> bool {
        // Item belongs to from_character
        if &ItemState::Owned(self.from_character.to_string())
            == world.items().get(&self.item).unwrap().state()
        {
            // Characters are in the same scene
            if world
                .characters()
                .get(&self.from_character)
                .unwrap()
                .scene()
                == world.characters().get(&self.to_character).unwrap().scene()
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn trigger(&mut self, world: &mut dyn World) {
        if let Some(world_update) = self.world_update.as_ref() {
            (world_update)(&self, world)
        }

        world
            .items_mut()
            .get_mut(&self.item)
            .unwrap()
            .set_state(if self.consume {
                ItemState::Unassigned
            } else {
                ItemState::Owned(self.to_character.to_string())
            });
    }

    fn translation_base(&self) -> String {
        format!(
            "{}_{}_{}_to_{}",
            self.from_character, self.name, self.item, self.to_character
        )
    }

    fn action_text(&self, world: &dyn World) -> String {
        self.make_action_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn success_text(&self, world: &dyn World) -> String {
        self.make_success_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn fail_text(&self, world: &dyn World) -> String {
        self.make_fail_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }
}

impl Give {
    pub fn new<SN, SFC, STC, SI>(
        name: SN,
        from_character: SFC,
        to_character: STC,
        item: SI,
        consume: bool,
        roles: Vec<&'static str>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self
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
            consume,
            roles,
            world_update,
            make_action_text,
            make_success_text,
            make_fail_text,
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
    consume: bool,
    roles: Vec<&'static str>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
}

impl fmt::Debug for UseItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("UseItem({})", self.name()))
            .field("character", &self.character)
            .field("item", &self.item)
            .field("consume", &self.item)
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

    fn roles(&self) -> Vec<&'static str> {
        self.roles.clone()
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

    fn can_be_triggered(&self, world: &dyn World) -> bool {
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }
        world.items().get(&self.item).unwrap().state()
            == &ItemState::Owned(self.character.to_string())
    }

    fn trigger(&mut self, world: &mut dyn World) {
        if let Some(world_update) = self.world_update.as_ref() {
            (world_update)(&self, world)
        }

        if self.consume {
            world
                .items_mut()
                .get_mut(&self.item)
                .unwrap()
                .set_state(ItemState::Unassigned);
        }
    }

    fn translation_base(&self) -> String {
        format!("{}_{}_{}", self.character, self.name, self.item)
    }

    fn action_text(&self, world: &dyn World) -> String {
        self.make_action_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn success_text(&self, world: &dyn World) -> String {
        self.make_success_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn fail_text(&self, world: &dyn World) -> String {
        self.make_fail_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }
}

impl UseItem {
    pub fn new<SN, SC, SI>(
        name: SN,
        character: SC,
        item: SI,
        consume: bool,
        roles: Vec<&'static str>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self
    where
        SN: ToString,
        SC: ToString,
        SI: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            item: item.to_string(),
            consume,
            roles,
            world_update,
            custom_condition,
            make_action_text,
            make_success_text,
            make_fail_text,
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
    from_scenes: Vec<&'static str>,
    to_scene: String,
    roles: Vec<&'static str>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("Move({})", self.name()))
            .field("character", &self.character)
            .field("to_scene", &self.to_scene)
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

    fn roles(&self) -> Vec<&'static str> {
        self.roles.clone()
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

    fn can_be_triggered(&self, world: &dyn World) -> bool {
        // Check custom condition
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }

        if let Some(scene) = world.characters().get(&self.character).unwrap().scene() {
            self.from_scenes
                .iter()
                .map(|e| e.to_string())
                .any(|e| &e == scene)
        } else {
            false
        }
    }

    fn trigger(&mut self, world: &mut dyn World) {
        world
            .characters_mut()
            .get_mut(&self.character)
            .unwrap()
            .set_scene(Some(self.to_scene.to_string()));
    }

    fn translation_base(&self) -> String {
        format!("{}_{}_to_{}", self.character, self.name, self.to_scene)
    }

    fn action_text(&self, world: &dyn World) -> String {
        self.make_action_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn success_text(&self, world: &dyn World) -> String {
        self.make_success_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn fail_text(&self, world: &dyn World) -> String {
        self.make_fail_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }
}

impl Move {
    pub fn new<SN, SC, SS>(
        name: SN,
        character: SC,
        from_scenes: Vec<&'static str>,
        to_scene: SS,
        roles: Vec<&'static str>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self
    where
        SN: ToString,
        SC: ToString,
        SS: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            from_scenes,
            to_scene: to_scene.to_string(),
            roles,
            custom_condition,
            make_action_text,
            make_success_text,
            make_fail_text,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &str {
        &self.character
    }
}

#[derive(Default)]
pub struct Void {
    id: Uuid,
    name: String,
    character: String,
    item: Option<String>,
    scenes: Option<Vec<&'static str>>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
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

    fn roles(&self) -> Vec<&'static str> {
        vec!["void"]
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

    fn can_be_triggered(&self, world: &dyn World) -> bool {
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }

        if let Some(scenes) = self.scenes.as_ref() {
            if let Some(scene) = world.characters().get(&self.character).unwrap().scene() {
                if scenes.iter().map(|e| e.to_string()).any(|e| &e == scene) {
                    return true;
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            true
        }
    }

    fn trigger(&mut self, world: &mut dyn World) {
        if let Some(world_update) = self.world_update.as_ref() {
            (world_update)(&self, world)
        }
    }

    fn translation_base(&self) -> String {
        if let Some(item) = self.item.as_ref() {
            format!("{}_{}_{}", self.character, self.name, item)
        } else {
            format!("{}_{}", self.character, self.name)
        }
    }

    fn action_text(&self, world: &dyn World) -> String {
        self.make_action_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn success_text(&self, world: &dyn World) -> String {
        self.make_success_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }

    fn fail_text(&self, world: &dyn World) -> String {
        self.make_fail_text
            .as_ref()
            .map(|e| (e)(self, world))
            .unwrap_or_else(String::new)
    }
}

impl Void {
    pub fn new<SN, SC, SI>(
        name: SN,
        character: SC,
        item: Option<SI>,
        scenes: Option<Vec<&'static str>>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_action_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_success_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_fail_text: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self
    where
        SN: ToString,
        SI: ToString,
        SC: ToString,
    {
        Self {
            name: name.to_string(),
            character: character.to_string(),
            item: item.map(|e| e.to_string()),
            scenes,
            world_update,
            custom_condition,
            make_action_text,
            make_success_text,
            make_fail_text,
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
