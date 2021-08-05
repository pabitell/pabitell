use pabitell_lib::{AsAny, Description, Id, Item, ItemState, Named, World};
use std::any::Any;
use uuid::Uuid;

use crate::translations::get_message;

macro_rules! simple_item {
    ($class_name: ident, $name: literal, $roles: expr) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: Uuid,
            state: ItemState,
        }

        impl Id for $class_name {
            fn id(&self) -> &Uuid {
                &self.id
            }

            fn set_id(&mut self, id: Uuid) {
                self.id = id;
            }

            fn roles(&self) -> Vec<&'static str> {
                $roles
            }
        }

        impl Named for $class_name {
            fn name(&self) -> &'static str {
                $name
            }
        }

        impl Description for $class_name {
            fn long(&self, world: &dyn World) -> String {
                get_message(
                    &format!("{}-{}-long", world.name(), $name),
                    world.lang(),
                    None,
                )
            }

            fn short(&self, world: &dyn World) -> String {
                get_message(
                    &format!("{}-{}-short", world.name(), $name),
                    world.lang(),
                    None,
                )
            }
        }

        impl AsAny for $class_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        impl Item for $class_name {
            fn state(&self) -> &ItemState {
                &self.state
            }

            fn set_state(&mut self, state: ItemState) {
                self.state = state;
            }
        }
    };
}

simple_item!(SandCake, "sand_cake", vec![]);
simple_item!(Flour, "flour", vec!["ingredient", "accepted"]);
simple_item!(Milk, "milk", vec!["ingredient", "accepted"]);
simple_item!(Egg, "egg", vec!["ingredient", "accepted"]);
simple_item!(Suggar, "suggar", vec!["ingredient", "accepted"]);
simple_item!(Salt, "salt", vec!["ingredient", "accepted"]);
simple_item!(Jam, "jam", vec!["ingredient", "rejected"]);
simple_item!(Cheese, "cheese", vec!["ingredient", "accepted"]);
simple_item!(Bacon, "bacon", vec!["ingredient", "accepted"]);
simple_item!(Peanuts, "peanuts", vec!["ingredient", "accepted"]);
simple_item!(Cucumber, "cucumber", vec!["ingredient", "accepted"]);
simple_item!(Bones, "bones", vec!["ingredient", "accepted"]);
simple_item!(FourMice, "four_mice", vec!["ingredient", "accepted"]);
simple_item!(Sausages, "sausages", vec!["ingredient", "accepted"]);
simple_item!(
    WhippedCream,
    "whipped_cream",
    vec!["ingredient", "accepted"]
);
simple_item!(Onion, "onion", vec!["ingredient", "accepted"]);
simple_item!(Chocolate, "chocolate", vec!["ingredient", "accepted"]);
simple_item!(Sauce, "sauce", vec!["ingredient", "accepted"]);
simple_item!(Garlic, "garlic", vec!["ingredient", "accepted"]);
simple_item!(Pepper, "pepper", vec!["ingredient", "accepted"]);
simple_item!(Lard, "lard", vec!["ingredient", "accepted"]);
simple_item!(Candy, "candy", vec!["ingredient", "accepted"]);
simple_item!(Greaves, "greaves", vec!["ingredient", "accepted"]);
simple_item!(Cinnamon, "cinnamon", vec!["ingredient", "accepted"]);
simple_item!(Porridge, "porridge", vec!["ingredient", "accepted"]);
simple_item!(
    CottageCheese,
    "cottage_cheese",
    vec!["ingredient", "accepted"]
);
simple_item!(GingerBread, "ginger_bread", vec!["ingredient", "accepted"]);
simple_item!(Vinegar, "vinegar", vec!["ingredient", "accepted"]);
simple_item!(GooseHead, "goose_head", vec!["ingredient", "accepted"]);
simple_item!(Cocoa, "cocoa", vec!["ingredient", "accepted"]);
simple_item!(Cabbadge, "cabbadge", vec!["ingredient", "accepted"]);
simple_item!(Raisins, "raisins", vec!["ingredient", "accepted"]);
simple_item!(Bread, "bread", vec!["ingredient", "rejected"]);

simple_item!(Marbles, "marbles", vec!["toy"]);
simple_item!(Ball, "ball", vec!["toy"]);
simple_item!(Dice, "dice", vec!["toy"]);

simple_item!(BadDog, "bad_dog", vec!["animal"]);

simple_item!(Soup, "soup", vec!["meal"]);
simple_item!(Meat, "meat", vec!["meal"]);
simple_item!(Pie, "pie", vec!["meal"]);
simple_item!(Dumplings, "dumplings", vec!["meal"]);
