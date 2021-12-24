use anyhow::anyhow;
use pabitell_lib::{scene_with_dialog, AsAny, Named, World};
use serde_json::Value;
use std::any::Any;

use crate::translations::get_message;

scene_with_dialog!(Home, "home", []);
scene_with_dialog!(Walk, "walk", []);
scene_with_dialog!(BackHome, "back_home", []);
