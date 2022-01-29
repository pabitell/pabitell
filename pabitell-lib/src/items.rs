// Note that get_message function should be provided
#[macro_export]
macro_rules! simple_item {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: uuid::Uuid,
            state: $crate::ItemState,
        }

        impl $crate::Named for $class_name {
            fn name(&self) -> &'static str {
                $name
            }
        }

        impl $crate::Tagged for $class_name {
            fn get_tags(&self) -> Vec<String> {
                #[allow(unused_mut)]
                let mut res: Vec<String> = vec![];
                $(
                    res.push($tag.into());
                )*
                res
            }
        }

        impl $crate::Description for $class_name {
            fn long(&self, world: &dyn $crate::World) -> String {
                get_message(
                    &format!("{}-{}-long", world.name(), $name),
                    world.lang(),
                    None,
                )
            }

            fn short(&self, world: &dyn $crate::World) -> String {
                get_message(
                    &format!("{}-{}-short", world.name(), $name),
                    world.lang(),
                    None,
                )
            }
        }

        impl $crate::AsAny for $class_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        impl $crate::Item for $class_name {
            fn state(&self) -> &$crate::ItemState {
                &self.state
            }

            fn set_state(&mut self, state: $crate::ItemState) {
                self.state = state;
            }
        }

        impl $crate::Dumpable for $class_name {
            fn dump(&self) -> serde_json::Value {
                serde_json::json!(
                    {"state": self.state.dump(), "name": self.name()}
                )
            }
            fn load(&mut self, data: serde_json::Value) -> anyhow::Result<()> {
                if let serde_json::Value::Object(mut object) = data {
                    let state_json = object.remove("state").ok_or_else(|| anyhow::anyhow!("Wrong format of item '{}'", self.name()))?;
                    self.state.load(state_json)?;
                    Ok(())
                } else{
                    Err(anyhow::anyhow!("Wrong format of item '{}'", self.name()))
                }
            }
        }

    };
}
