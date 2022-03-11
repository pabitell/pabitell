use pabitell_lib::{simple_item, Named};
use std::any::Any;

use crate::translations::get_message;

simple_item!(SandCake, "sand_cake", []);

simple_item!(Flour, "flour", ["ingredient", "accepted", "batch1"]);
simple_item!(Milk, "milk", ["ingredient", "accepted", "batch1"]);
simple_item!(Egg, "egg", ["ingredient", "accepted", "batch1"]);

simple_item!(Sugar, "sugar", ["ingredient", "accepted", "batch2"]);
simple_item!(Salt, "salt", ["ingredient", "accepted", "batch2"]);
simple_item!(Butter, "butter", ["ingredient", "accepted", "batch2"]);
simple_item!(Jam, "jam", ["ingredient", "rejected", "batch2"]);
simple_item!(Cheese, "cheese", ["ingredient", "accepted", "batch2"]);

simple_item!(Bacon, "bacon", ["ingredient", "accepted", "batch3"]);
simple_item!(Peanuts, "peanuts", ["ingredient", "accepted", "batch3"]);
simple_item!(Cucumber, "cucumber", ["ingredient", "accepted", "batch3"]);
simple_item!(Bones, "bones", ["ingredient", "accepted", "batch3"]);

simple_item!(FourMice, "four_mice", ["ingredient", "accepted", "batch4"]);
simple_item!(Sausages, "sausages", ["ingredient", "accepted", "batch4"]);
simple_item!(
    WhippedCream,
    "whipped_cream",
    ["ingredient", "accepted", "batch4"]
);
simple_item!(Onion, "onion", ["ingredient", "accepted", "batch4"]);
simple_item!(Chocolate, "chocolate", ["ingredient", "accepted", "batch4"]);
simple_item!(Sauce, "sauce", ["ingredient", "accepted", "batch4"]);

simple_item!(Garlic, "garlic", ["ingredient", "accepted", "batch5"]);
simple_item!(Pepper, "pepper", ["ingredient", "accepted", "batch5"]);
simple_item!(Lard, "lard", ["ingredient", "accepted", "batch5"]);
simple_item!(Candy, "candy", ["ingredient", "accepted", "batch5"]);
simple_item!(Greaves, "greaves", ["ingredient", "accepted", "batch5"]);
simple_item!(Cinnamon, "cinnamon", ["ingredient", "accepted", "batch5"]);
simple_item!(Porridge, "porridge", ["ingredient", "accepted", "batch5"]);
simple_item!(
    CottageCheese,
    "cottage_cheese",
    ["ingredient", "accepted", "batch5"]
);

simple_item!(
    GingerBread,
    "ginger_bread",
    ["ingredient", "accepted", "batch6"]
);
simple_item!(Vinegar, "vinegar", ["ingredient", "accepted", "batch6"]);
simple_item!(Cocoa, "cocoa", ["ingredient", "accepted", "batch6"]);
simple_item!(Cabbage, "cabbage", ["ingredient", "accepted", "batch6"]);
simple_item!(
    GooseHead,
    "goose_head",
    ["ingredient", "accepted", "batch6"]
);
simple_item!(Raisins, "raisins", ["ingredient", "accepted", "batch6"]);
simple_item!(Bread, "bread", ["ingredient", "rejected", "batch6"]);

simple_item!(Marbles, "marbles", ["toy"]);
simple_item!(Ball, "ball", ["toy"]);
simple_item!(Dice, "dice", ["toy"]);

simple_item!(BadDog, "bad_dog", ["animal"]);

simple_item!(Soup, "soup", ["meal"]);
simple_item!(Meat, "meat", ["meal"]);
simple_item!(Pie, "pie", ["meal"]);
simple_item!(Dumplings, "dumplings", ["meal"]);
