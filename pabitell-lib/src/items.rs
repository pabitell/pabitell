#[macro_export]
macro_rules! simple_item {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            state: $crate::ItemState,
            last_event: Option<usize>,
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

        impl $crate::Description for $class_name {}

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

            fn last_event(&self) -> Option<usize> {
                self.last_event
            }

            fn set_last_event(&mut self, event: usize) {
                self.last_event = Some(event);
            }
        }

        impl $crate::Dumpable for $class_name {
            fn dump(&self) -> serde_json::Value {
                serde_json::json!(
                    {
                        "state": self.state.dump(),
                        "name": self.name(),
                        "last_event": self.last_event,
                    }
                )
            }
            fn load(&mut self, data: serde_json::Value) -> anyhow::Result<()> {
                if let serde_json::Value::Object(mut object) = data {
                    let state_json = object.remove("state").ok_or_else(|| anyhow::anyhow!("Wrong format of item '{}'", self.name()))?;
                    let last_event_json = object.remove("last_event").ok_or_else(|| anyhow::anyhow!("Wrong format of item '{}'", self.name()))?;
                    if let Some(last_event) = last_event_json.as_u64() {
                        self.last_event = Some(last_event as usize);
                    } else if last_event_json.is_null() {
                        self.last_event = None;
                    } else {
                        return Err(anyhow::anyhow!("Wrong format of item '{}'", self.name()));
                    }
                    self.state.load(state_json)?;
                    Ok(())
                } else{
                    Err(anyhow::anyhow!("Wrong format of item '{}'", self.name()))
                }
            }
        }

        impl $crate::Clean for $class_name {
            fn clean(&mut self) {
                self.state = $crate::ItemState::Unassigned;
                self.last_event = None;
            }
        }

    };
}
