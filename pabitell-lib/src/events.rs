use crate::{AsAny, Description, Event, Id, ItemState, Named, World};
use std::{any::Any, fmt};
use uuid::Uuid;

#[derive(Default)]
pub struct Pick {
    id: Uuid,
    name: &'static str,
    character: &'static str,
    item: &'static str,
    consume: bool,
    roles: Vec<&'static str>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
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

impl Named for Pick {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Description for Pick {
    fn long(&self, world: &dyn World) -> String {
        if let Some(long) = self.make_long.as_ref() {
            (long)(self, world)
        } else {
            String::new()
        }
    }

    fn short(&self, world: &dyn World) -> String {
        if let Some(short) = self.make_short.as_ref() {
            (short)(self, world)
        } else {
            String::new()
        }
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
    fn can_be_triggered(&self, world: &dyn World) -> bool {
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }

        if let Some(scene) = world.characters().get(self.character).unwrap().scene() {
            if &ItemState::InScene(scene) == world.items().get(self.item).unwrap().state() {
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
            .get_mut(self.item)
            .unwrap()
            .set_state(if self.consume {
                ItemState::Unassigned
            } else {
                ItemState::Owned(self.character)
            });
    }
}

impl Pick {
    pub fn new(
        name: &'static str,
        character: &'static str,
        item: &'static str,
        consume: bool,
        roles: Vec<&'static str>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self {
        Self {
            name,
            character,
            item,
            consume,
            roles,
            custom_condition,
            world_update,
            make_long,
            make_short,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &'static str {
        self.character
    }

    pub fn item(&self) -> &'static str {
        self.item
    }
}

#[derive(Default)]
pub struct Give {
    id: Uuid,
    name: &'static str,
    from_character: &'static str,
    to_character: &'static str,
    item: &'static str,
    consume: bool,
    roles: Vec<&'static str>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
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

impl Named for Give {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Description for Give {
    fn long(&self, world: &dyn World) -> String {
        if let Some(long) = self.make_long.as_ref() {
            (long)(self, world)
        } else {
            String::new()
        }
    }

    fn short(&self, world: &dyn World) -> String {
        if let Some(short) = self.make_short.as_ref() {
            (short)(self, world)
        } else {
            String::new()
        }
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
    fn can_be_triggered(&self, world: &dyn World) -> bool {
        // Item belongs to from_character
        if &ItemState::Owned(self.from_character) == world.items().get(self.item).unwrap().state() {
            // Characters are in the same scene
            if world.characters().get(self.from_character).unwrap().scene()
                == world.characters().get(self.to_character).unwrap().scene()
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
            .get_mut(self.item)
            .unwrap()
            .set_state(if self.consume {
                ItemState::Unassigned
            } else {
                ItemState::Owned(self.to_character)
            });
    }
}

impl Give {
    pub fn new(
        name: &'static str,
        from_character: &'static str,
        to_character: &'static str,
        item: &'static str,
        consume: bool,
        roles: Vec<&'static str>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self {
        Self {
            name,
            from_character,
            to_character,
            item,
            consume,
            roles,
            world_update,
            make_long,
            make_short,
            ..Default::default()
        }
    }

    pub fn from_character(&self) -> &'static str {
        self.from_character
    }

    pub fn to_character(&self) -> &'static str {
        self.to_character
    }

    pub fn item(&self) -> &'static str {
        self.item
    }
}

#[derive(Default)]
pub struct UseItem {
    id: Uuid,
    name: &'static str,
    character: &'static str,
    item: &'static str,
    consume: bool,
    roles: Vec<&'static str>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
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

impl Named for UseItem {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Description for UseItem {
    fn long(&self, world: &dyn World) -> String {
        if let Some(long) = self.make_long.as_ref() {
            (long)(self, world)
        } else {
            String::new()
        }
    }

    fn short(&self, world: &dyn World) -> String {
        if let Some(short) = self.make_short.as_ref() {
            (short)(self, world)
        } else {
            String::new()
        }
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
    fn can_be_triggered(&self, world: &dyn World) -> bool {
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }
        world.items().get(self.item).unwrap().state() == &ItemState::Owned(self.character)
    }

    fn trigger(&mut self, world: &mut dyn World) {
        if let Some(world_update) = self.world_update.as_ref() {
            (world_update)(&self, world)
        }

        if self.consume {
            world
                .items_mut()
                .get_mut(self.item)
                .unwrap()
                .set_state(ItemState::Unassigned);
        }
    }
}

impl UseItem {
    pub fn new(
        name: &'static str,
        character: &'static str,
        item: &'static str,
        consume: bool,
        roles: Vec<&'static str>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self {
        Self {
            name,
            character,
            item,
            consume,
            roles,
            world_update,
            custom_condition,
            make_short,
            make_long,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &'static str {
        self.character
    }

    pub fn item(&self) -> &'static str {
        self.item
    }
}

#[derive(Default)]
pub struct Move {
    id: Uuid,
    name: &'static str,
    character: &'static str,
    from_scenes: Vec<&'static str>,
    to_scene: &'static str,
    roles: Vec<&'static str>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
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

impl Named for Move {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Description for Move {
    fn long(&self, world: &dyn World) -> String {
        if let Some(long) = self.make_long.as_ref() {
            (long)(self, world)
        } else {
            String::new()
        }
    }

    fn short(&self, world: &dyn World) -> String {
        if let Some(short) = self.make_short.as_ref() {
            (short)(self, world)
        } else {
            String::new()
        }
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
    fn can_be_triggered(&self, world: &dyn World) -> bool {
        // Check custom condition
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }

        if let Some(scene) = world
            .characters()
            .get(self.character)
            .unwrap()
            .scene()
            .as_ref()
        {
            self.from_scenes.contains(scene)
        } else {
            false
        }
    }

    fn trigger(&mut self, world: &mut dyn World) {
        world
            .characters_mut()
            .get_mut(self.character)
            .unwrap()
            .set_scene(Some(self.to_scene));
    }
}

impl Move {
    pub fn new(
        name: &'static str,
        character: &'static str,
        from_scenes: Vec<&'static str>,
        to_scene: &'static str,
        roles: Vec<&'static str>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self {
        Self {
            name,
            character,
            from_scenes,
            to_scene,
            roles,
            custom_condition,
            make_long,
            make_short,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &'static str {
        self.character
    }
}

#[derive(Default)]
pub struct Void {
    id: Uuid,
    name: &'static str,
    character: &'static str,
    item: Option<&'static str>,
    scenes: Option<Vec<&'static str>>,
    world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
    custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
    make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
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

impl Named for Void {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Description for Void {
    fn long(&self, world: &dyn World) -> String {
        if let Some(long) = self.make_long.as_ref() {
            (long)(self, world)
        } else {
            String::new()
        }
    }

    fn short(&self, world: &dyn World) -> String {
        if let Some(short) = self.make_short.as_ref() {
            (short)(self, world)
        } else {
            String::new()
        }
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
    fn can_be_triggered(&self, world: &dyn World) -> bool {
        if let Some(custom_condition) = self.custom_condition.as_ref() {
            if !(custom_condition)(self, world) {
                return false;
            }
        }

        if let Some(scenes) = self.scenes.as_ref() {
            if let Some(scene) = world.characters().get(self.character).unwrap().scene() {
                if scenes.contains(&scene) {
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
}

impl Void {
    pub fn new(
        name: &'static str,
        character: &'static str,
        item: Option<&'static str>,
        scenes: Option<Vec<&'static str>>,
        world_update: Option<Box<dyn Fn(&Self, &mut dyn World)>>,
        custom_condition: Option<Box<dyn Fn(&Self, &dyn World) -> bool>>,
        make_short: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
        make_long: Option<Box<dyn Fn(&Self, &dyn World) -> String>>,
    ) -> Self {
        Self {
            name,
            character,
            item,
            scenes,
            world_update,
            custom_condition,
            make_short,
            make_long,
            ..Default::default()
        }
    }

    pub fn character(&self) -> &'static str {
        self.character
    }

    pub fn item(&self) -> Option<&'static str> {
        self.item
    }
}
