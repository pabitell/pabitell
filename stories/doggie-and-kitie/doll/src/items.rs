use anyhow::{anyhow, Result};
use pabitell_lib::{
    simple_item, AsAny, Description, Dumpable, Id, Item, ItemState, Named, Tagged, World,
};
use serde_json::{json, Value};
use std::any::Any;

use crate::translations::get_message;

simple_item!(Doll, "doll", []);

simple_item!(SmallBall, "small_ball", ["doggie_pick"]);
simple_item!(SandMoulds, "sand_moulds", ["doggie_pick"]);
simple_item!(Beads, "beads", ["doggie_pick"]);
simple_item!(ColoredCubes, "colored_cubes", ["doggie_pick"]);
simple_item!(Crockery, "crockery", ["doggie_pick"]);
simple_item!(SmallStove, "small_stove", ["doggie_pick"]);
simple_item!(SmallChair, "small_chair", ["doggie_pick"]);
simple_item!(Pictures, "pictures", ["doggie_pick"]);
simple_item!(Whistle, "whistle", ["doggie_pick"]);
simple_item!(Spoon, "spoon", ["doggie_pick"]);
simple_item!(SmallShovel, "small_shovel", ["doggie_pick"]);
simple_item!(WoodenHouses, "wooden_houses", ["doggie_pick"]);
simple_item!(WoodenTrees, "wooden_trees", ["doggie_pick"]);
simple_item!(WoodenAnimals, "wooden_animals", ["doggie_pick"]);

simple_item!(Bucket, "bucket", ["kitie_pick"]);
simple_item!(WateringCan, "watering_can", ["kitie_pick"]);
simple_item!(BuildingCubes, "building_cubes", ["kitie_pick"]);
simple_item!(Slippers, "slippers", ["kitie_pick"]);
simple_item!(Stockings, "stockings", ["kitie_pick"]);
simple_item!(FairyTaleBook, "fairy_tale_book", ["kitie_pick"]);
simple_item!(Hanky, "hanky", ["kitie_pick"]);
simple_item!(ColoredCloth, "colored_cloth", ["kitie_pick"]);
simple_item!(Threads, "threads", ["kitie_pick"]);
simple_item!(Needlework, "needlework", ["kitie_pick"]);
simple_item!(RoundNeedle, "round_needle", ["kitie_pick"]);
simple_item!(SmallDoll, "small_doll", ["kitie_pick"]);
simple_item!(FeatherBall, "feather_ball", ["kitie_pick"]);
simple_item!(ColoredPapers, "colored_papers", ["kitie_pick"]);
simple_item!(ThrowingRing, "throwing_ring", ["kitie_pick"]);
simple_item!(Cuttlery, "cuttlery", ["kitie_pick"]);
