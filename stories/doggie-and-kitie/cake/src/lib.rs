pub mod characters;
pub mod events;
pub mod items;
pub mod scenes;
pub mod translations;

use anyhow::Result;
#[cfg(feature = "with_world_setup")]
use pabitell_lib::ItemState;
use pabitell_lib::{
    translations::get_available_locales, Character, Description, Id, Item, Named, Scene, World,
    WorldBuilder,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::translations::{get_message, RESOURCES};

const DEFAULT_LANG: &str = "cs";

#[derive(Debug, Default)]
pub struct DortWorld {
    id: Uuid,
    lang: String,
    items: HashMap<String, Box<dyn Item>>,
    characters: HashMap<String, Box<dyn Character>>,
    scenes: HashMap<String, Box<dyn Scene>>,
}

struct DortWorldDescription;
impl Named for DortWorldDescription {
    fn name(&self) -> &str {
        "description"
    }
}

impl Description for DortWorldDescription {
    fn long(&self, world: &Box<dyn World>) -> String {
        get_message(&format!("{}.long", world.name()), world.lang(), None)
    }

    fn short(&self, world: &Box<dyn World>) -> String {
        get_message(&format!("{}.short", world.name()), world.lang(), None)
    }
}

#[derive(Default)]
struct DortWorldBuilder {
    items: Vec<Box<dyn Item>>,
    characters: Vec<Box<dyn Character>>,
    scenes: Vec<Box<dyn Scene>>,
}

impl WorldBuilder<DortWorld> for DortWorldBuilder {
    fn character(mut self, character: Box<dyn Character>) -> Self {
        self.characters.push(character);
        self
    }

    fn item(mut self, item: Box<dyn Item>) -> Self {
        self.items.push(item);
        self
    }

    fn scene(mut self, scene: Box<dyn Scene>) -> Self {
        self.scenes.push(scene);
        self
    }

    fn build(self) -> Result<DortWorld> {
        Ok(DortWorld {
            lang: DEFAULT_LANG.into(),
            characters: self
                .characters
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),
            items: self
                .items
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),
            scenes: self
                .scenes
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),

            ..Default::default()
        })
    }

    fn make_world() -> Result<DortWorld> {
        Self::default()
            .scene(Box::new(scenes::PlayGround::default()))
            .scene(Box::new(scenes::Kitchen::default()))
            .scene(Box::new(scenes::Garden::default()))
            .scene(Box::new(scenes::ChildrenHouse::default()))
            .scene(Box::new(scenes::ChildrenGarden::default()))
            .scene(Box::new(scenes::WayHome::default()))
            .character(Box::new(characters::Kitie::default()))
            .character(Box::new(characters::Doggie::default()))
            .item(Box::new(items::SandCake::default()))
            .item(Box::new(items::Flour::default()))
            .item(Box::new(items::Milk::default()))
            .item(Box::new(items::Egg::default()))
            .item(Box::new(items::Suggar::default()))
            .item(Box::new(items::Salt::default()))
            .item(Box::new(items::Jam::default()))
            .item(Box::new(items::Cheese::default()))
            .item(Box::new(items::Bacon::default()))
            .item(Box::new(items::Peanuts::default()))
            .item(Box::new(items::Cucumber::default()))
            .item(Box::new(items::Bones::default()))
            .item(Box::new(items::FourMice::default()))
            .item(Box::new(items::Sausages::default()))
            .item(Box::new(items::WhippedCream::default()))
            .item(Box::new(items::Onion::default()))
            .item(Box::new(items::Chocolate::default()))
            .item(Box::new(items::Sauce::default()))
            .item(Box::new(items::Garlic::default()))
            .item(Box::new(items::Pepper::default()))
            .item(Box::new(items::Lard::default()))
            .item(Box::new(items::Candy::default()))
            .item(Box::new(items::Greaves::default()))
            .item(Box::new(items::Cinnamon::default()))
            .item(Box::new(items::Porridge::default()))
            .item(Box::new(items::CottageCheese::default()))
            .item(Box::new(items::GingerBread::default()))
            .item(Box::new(items::Vinegar::default()))
            .item(Box::new(items::GooseHead::default()))
            .item(Box::new(items::Cocoa::default()))
            .item(Box::new(items::Cabbadge::default()))
            .item(Box::new(items::Raisins::default()))
            .item(Box::new(items::Bread::default()))
            .item(Box::new(items::Marbles::default()))
            .item(Box::new(items::Ball::default()))
            .item(Box::new(items::Dice::default()))
            .item(Box::new(items::BadDog::default()))
            .item(Box::new(items::Soup::default()))
            .item(Box::new(items::Meat::default()))
            .item(Box::new(items::Dumplings::default()))
            .item(Box::new(items::Pie::default()))
            .build()
    }
}

impl Id for DortWorld {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
    fn roles(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl Named for DortWorld {
    fn name(&self) -> &str {
        "povidani_o_pejskovi_a_kocicce-dort"
    }
}

impl World for DortWorld {
    fn characters(&self) -> &HashMap<String, Box<dyn Character>> {
        &self.characters
    }

    fn characters_mut(&mut self) -> &mut HashMap<String, Box<dyn Character>> {
        &mut self.characters
    }

    fn scenes(&self) -> &HashMap<String, Box<dyn Scene>> {
        &self.scenes
    }

    fn scenes_mut(&mut self) -> &mut HashMap<String, Box<dyn Scene>> {
        &mut self.scenes
    }

    fn items(&self) -> &HashMap<String, Box<dyn Item>> {
        &self.items
    }

    fn items_mut(&mut self) -> &mut HashMap<String, Box<dyn Item>> {
        &mut self.items
    }

    fn description(&self) -> Box<dyn Description> {
        Box::new(DortWorldDescription)
    }

    fn lang(&self) -> &str {
        &self.lang
    }

    fn set_lang(&mut self, lang: &str) -> bool {
        if let Ok(locales) = get_available_locales(&RESOURCES) {
            if locales.iter().any(|l| l.to_string() == lang) {
                self.lang = lang.into();
                return true;
            }
        }
        false
    }

    #[cfg(feature = "with_world_setup")]
    fn setup(&mut self) {
        self.randomize_ids();

        self.characters_mut()
            .values_mut()
            .for_each(|c| c.set_scene(Some("playground")));

        self.items_mut().values_mut().for_each(|i| {
            i.set_state(match i.name() {
                "sand_cake" => ItemState::InScene("playground"),
                "bad_dog" => ItemState::InScene("garden"),
                _ => {
                    if i.roles().contains(&"ingredient") {
                        ItemState::InScene("kitchen")
                    } else if i.roles().contains(&"toy") {
                        ItemState::InScene("children_garden")
                    } else if i.roles().contains(&"meal") {
                        ItemState::InScene("children_house")
                    } else {
                        ItemState::Unassigned
                    }
                }
            })
        });
    }
}

#[cfg(test)]
pub mod tests {
    use pabitell_lib::{Description, Event, Id, ItemState, World, WorldBuilder};
    use uuid::Uuid;

    use super::events;
    use crate::{characters, DortWorld, DortWorldBuilder};

    #[cfg(feature = "with_world_setup")]
    pub fn prepare_world() -> DortWorld {
        let mut world = DortWorldBuilder::make_world().unwrap();
        world.setup();
        world
    }

    #[test]
    #[cfg(feature = "with_world_setup")]
    fn setup() {
        let world = prepare_world();
        assert_eq!(
            world.characters().get("kitie").unwrap().scene(),
            Some("playground")
        );
        assert_eq!(
            world.characters().get("doggie").unwrap().scene(),
            Some("playground")
        );
        assert_eq!(
            world.items().get("sand_cake").unwrap().state(),
            &ItemState::InScene("playground")
        );
        assert_eq!(
            world.items().get("milk").unwrap().state(),
            &ItemState::InScene("kitchen")
        );
        assert_eq!(
            world.items().get("jam").unwrap().state(),
            &ItemState::InScene("kitchen")
        );
        assert_eq!(
            world.items().get("bread").unwrap().state(),
            &ItemState::InScene("kitchen")
        );
        assert_eq!(
            world.items().get("raisins").unwrap().state(),
            &ItemState::InScene("kitchen")
        );
        assert_eq!(
            world.items().get("ball").unwrap().state(),
            &ItemState::InScene("children_garden")
        );
        assert_eq!(
            world.items().get("bad_dog").unwrap().state(),
            &ItemState::InScene("garden")
        );
        assert_eq!(
            world.items().get("dumplings").unwrap().state(),
            &ItemState::InScene("children_house")
        );
    }

    #[cfg(feature = "with_world_setup")]
    #[test]
    fn workflow() {
        let mut world: Box<dyn World> = Box::new(prepare_world());
        // pick sand cake
        let event = events::make_pick("pick_sand_cake", "kitie", "sand_cake", false);
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_pick("pick_sand_cake", "doggie", "sand_cake", false);
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        // give_and_consume sand cake
        let event = events::make_give("give_sand_cake", "kitie", "doggie", "sand_cake", true);
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_give("give_sand_cake", "kitie", "doggie", "sand_cake", true);
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        // move both characters to kitchen
        let event = events::make_move_to_kitchen("doggie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_kitchen("doggie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));
        let event = events::make_move_to_kitchen("kitie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_kitchen("kitie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        // Put thinkgs to cake
        let mut doggie = false;
        for (pick_name, use_name, item) in [
            ("pick_flour", "add_flour", "flour"),
            ("pick_milk", "add_milk", "milk"),
            ("pick_egg", "add_egg", "egg"),
            ("pick_suggar", "add_sugar", "suggar"),
            ("pick_salt", "add_salt", "salt"),
            ("pick_cheese", "add_cheese", "cheese"),
            ("pick_bacon", "add_bacon", "bacon"),
            ("pick_peanuts", "add_peanuts", "peanuts"),
            ("pick_cucumber", "add_cucumber", "cucumber"),
            ("pick_bones", "add_bones", "bones"),
            ("pick_four_mice", "add_four_mice", "four_mice"),
            ("pick_sausages", "add_sausages", "sausages"),
            ("pick_whipped_cream", "add_whipped_cream", "whipped_cream"),
            ("pick_onion", "add_onion", "onion"),
            ("pick_chocolate", "add_chocolate", "chocolate"),
            ("pick_sauce", "add_sauce", "sauce"),
            ("pick_garlic", "add_garlic", "garlic"),
            ("pick_pepper", "add_pepper", "pepper"),
            ("pick_lard", "add_lard", "lard"),
            ("pick_candy", "add_candy", "candy"),
            ("pick_greaves", "add_greaves", "greaves"),
            ("pick_cinnamon", "add_cinnamon", "cinnamon"),
            ("pick_porridge", "add_porridge", "porridge"),
            (
                "pick_cottage_cheese",
                "add_cottage_cheese",
                "cottage_cheese",
            ),
            ("pick_ginger_bread", "add_ginger_bread", "ginger_bread"),
            ("pick_vinegar", "add_vinegar", "vinegar"),
            ("pick_goose_head", "add_goose_head", "goose_head"),
            ("pick_cocoa", "add_cocoa", "cocoa"),
            ("pick_cabbadge", "add_cabbadge", "cabbadge"),
            ("pick_raisins", "add_raisins", "raisins"),
        ] {
            let event = events::make_pick(
                pick_name,
                if doggie { "doggie" } else { "kitie" },
                item,
                false,
            );
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            let event = events::make_pick(
                pick_name,
                if doggie { "doggie" } else { "kitie" },
                item,
                false,
            );
            assert!(!event.can_be_triggered(&world));
            assert!(!event.perform(&mut world));
            let event = events::make_use_item(
                use_name,
                if doggie { "doggie" } else { "kitie" },
                item,
                true,
            );
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            let event = events::make_use_item(
                use_name,
                if doggie { "doggie" } else { "kitie" },
                item,
                true,
            );
            assert!(!event.can_be_triggered(&world));
            assert!(!event.perform(&mut world));
            doggie = !doggie;
        }

        // Put disliked thing to cake
        let event = events::make_disliked_pick("add_jam", "kitie", "jam");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_disliked_pick("add_jam", "kitie", "jam");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_disliked_pick("add_bread", "doggie", "bread");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_disliked_pick("add_bread", "kitie", "bread");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));

        // move both characters to children's garden
        let event = events::make_move_to_children_garden("doggie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_children_garden("doggie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));
        let event = events::make_move_to_children_garden("kitie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_children_garden("kitie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        for (name, item) in [
            ("play_with_marbles", "marbles"),
            ("play_with_ball", "ball"),
            ("play_with_dice", "dice"),
        ] {
            let event =
                events::make_pick(name, if doggie { "doggie" } else { "kitie" }, item, true);
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            let event =
                events::make_pick(name, if doggie { "doggie" } else { "kitie" }, item, true);
            assert!(!event.can_be_triggered(&world));
            assert!(!event.perform(&mut world));
            doggie = !doggie;
        }

        // move both characters to garden
        let event = events::make_move_to_garden("doggie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_garden("doggie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));
        let event = events::make_move_to_garden("kitie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_garden("kitie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        // find big bad dog
        let event = events::make_find_bad_dog("doggie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_find_bad_dog("kitie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        // go to children house
        let event = events::make_move_to_children_house("doggie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_children_house("doggie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));
        let event = events::make_move_to_children_house("kitie");
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        let event = events::make_move_to_children_house("kitie");
        assert!(!event.can_be_triggered(&world));
        assert!(!event.perform(&mut world));

        for (name, item) in [
            ("eat_soup", "soup"),
            ("eat_meat", "meat"),
            ("eat_dumplings", "dumplings"),
            ("eat_pie", "pie"),
        ] {
            let event = events::make_eat_meal(name, "doggie", item);
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));

            let event = events::make_eat_meal(name, "kitie", item);
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Make sure that doggie and kitie reached final destination
        assert!(world.characters().get("doggie").unwrap().scene() == Some("way_home"));
        assert!(world.characters().get("kitie").unwrap().scene() == Some("way_home"));
    }

    #[test]
    fn languages() {
        let mut world = DortWorldBuilder::make_world().unwrap();
        for lang in vec!["cs", "en-US"] {
            assert!(world.set_lang(lang));
        }
    }
}
