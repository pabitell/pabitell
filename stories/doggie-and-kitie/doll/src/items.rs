use anyhow::{anyhow, Result};
use pabitell_lib::{
    simple_item, AsAny, Description, Dumpable, Id, Item, ItemState, Named, Tagged, World,
};
use serde_json::{json, Value};
use std::any::Any;

use crate::translations::get_message;

simple_item!(Doll, "doll", []);

simple_item!(SmallBall, "small_ball", ["doggie_pick"]);

simple_item!(Bucket, "bucket", ["kitie_pick"]);
