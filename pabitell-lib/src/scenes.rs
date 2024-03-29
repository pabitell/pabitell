#[macro_export]
macro_rules! scene_base {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: uuid::Uuid,
            location: Option<$crate::GeoLocation>,
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

        impl AsAny for $class_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }


        impl $crate::Dumpable for $class_name {
            fn dump(&self) -> serde_json::Value {
                serde_json::json!(
                    {
                        "name": self.name(),
                        "location": self.location,
                    }
                )
            }
            fn load(&mut self, data: serde_json::Value) -> anyhow::Result<()> {
                let location: Option<$crate::GeoLocation> = serde_json::from_value(data["location"].clone())?;
                self.location = location;
                Ok(())  // Scenes doesn't cotain any extras here
            }
        }

        impl $crate::Scene for $class_name {
            fn geo_location(&self) -> Option<$crate::GeoLocation> {
                self.location
            }

            fn set_geo_location(&mut self, location: Option<$crate::GeoLocation>) {
                self.location = location
            }
        }

        impl $crate::Clean for $class_name {
            fn clean(&mut self) {}
        }
    };
}

#[macro_export]
macro_rules! scene_no_music {
    ($class_name: ident) => {
        impl Music for $class_name {}
    };
}

#[macro_export]
macro_rules! scene_with_dialog {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: uuid::Uuid,
            dialog: usize,
            location: Option<$crate::GeoLocation>,
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

        impl AsAny for $class_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }


        impl $crate::Dumpable for $class_name {
            fn dump(&self) -> serde_json::Value {
                serde_json::json!(
                    {
                        "name": self.name(),
                        "dialog": self.dialog,
                        "location": self.location,
                    }
                )
            }

            fn load(&mut self, data: serde_json::Value) -> anyhow::Result<()> {
                if let Value::Number(number) = &data["dialog"] {
                    if let Some(dialog) = number.as_u64() {
                        self.dialog = dialog as usize;
                    } else {
                        return Err(anyhow!("Wrong dialog field '{}'", number));
                    }
                } else {
                    return Err(anyhow!("Scene format '{}'", self.name()));
                }

                let location: Option<$crate::GeoLocation> = serde_json::from_value(data["location"].clone())?;
                self.location = location;


                Ok(())
            }
        }

        impl $crate::Scene for $class_name {
            fn dialog(&self) -> Option<usize> {
                Some(self.dialog)
            }

            fn next_dialog(&mut self) {
                self.dialog += 1
            }

            fn geo_location(&self) -> Option<$crate::GeoLocation> {
                self.location
            }

            fn set_geo_location(&mut self, location: Option<$crate::GeoLocation>) {
                self.location = location
            }
        }

        impl $crate::Description for $class_name {
            fn long(&self, world: &dyn World) -> String {
                world.get_message(
                    &format!("{}-{}-long-{}", world.name(), self.name(), self.dialog),
                    None,
                )
            }

            fn short(&self, world: &dyn World) -> String {
                world.get_message(
                    &format!("{}-{}-short", world.name(), self.name()),
                    None,
                )
            }
        }

        impl $crate::Clean for $class_name {
            fn clean(&mut self) {
                self.dialog = 0;
            }
        }
    };
}
