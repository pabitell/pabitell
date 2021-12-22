#[macro_export]
macro_rules! scene_base {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: uuid::Uuid,
        }

        impl $crate::Id for $class_name {
            fn id(&self) -> &uuid::Uuid {
                &self.id
            }

            fn set_id(&mut self, id: uuid::Uuid) {
                self.id = id;
            }
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
                    {"name": self.name()}
                )
            }
            fn load(&mut self, data: serde_json::Value) -> anyhow::Result<()> {
                Ok(())  // Scenes doesn't cotain any extras here
            }
        }

        impl $crate::Scene for $class_name {}
    };
}
