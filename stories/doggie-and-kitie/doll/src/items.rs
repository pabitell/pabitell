use anyhow::{anyhow, Result};
use pabitell_lib::{AsAny, Description, Dumpable, Id, Item, ItemState, Named, Tagged, World};
use serde_json::{json, Value};
use std::any::Any;
use uuid::Uuid;

use crate::translations::get_message;

// TODO put it into pabitell_lib
macro_rules! simple_item {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
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
        }

        impl Named for $class_name {
            fn name(&self) -> &'static str {
                $name
            }
        }

        impl Tagged for $class_name {
            fn get_tags(&self) -> Vec<String> {
                #[allow(unused_mut)]
                let mut res: Vec<String> = vec![];
                $(
                    res.push($tag.into());
                )*
                res
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

        impl Dumpable for $class_name {
            fn dump(&self) -> Value {
                json!(
                    {"state": self.state.dump(), "name": self.name()}
                )
            }
            fn load(&mut self, data: Value) -> Result<()> {
                if let Value::Object(mut object) = data {
                    let state_json = object.remove("state").ok_or_else(|| anyhow!("Wrong format of item '{}'", self.name()))?;
                    self.state.load(state_json)?;
                    Ok(())
                } else{
                    Err(anyhow!("Wrong format of item '{}'", self.name()))
                }
            }
        }

    };
}

simple_item!(Doll, "doll", []);

simple_item!(Ball, "ball", ["doggie_pick"]);

simple_item!(Bucket, "bucket", ["kitie_pick"]);
