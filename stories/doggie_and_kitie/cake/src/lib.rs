pub mod characters;
pub mod events;
pub mod items;
pub mod narrator;
pub mod scenes;
pub mod translations;
pub mod world;

#[cfg(test)]
pub mod tests {
    use pabitell_lib::{
        events as lib_events, Description, Dumpable, Event, ItemState, Narrator, World,
        WorldBuilder,
    };
    use uuid::Uuid;

    use super::events;
    use crate::{
        characters, narrator,
        world::{CakeWorld, CakeWorldBuilder},
    };

    pub fn prepare_world() -> CakeWorld {
        let mut world = CakeWorldBuilder::make_world().unwrap();
        world.setup();
        world
    }

    #[test]
    fn setup() {
        let world = prepare_world();
        assert_eq!(
            world.characters().get("kitie").unwrap().scene(),
            &Some("playground".into())
        );
        assert_eq!(
            world.characters().get("doggie").unwrap().scene(),
            &Some("playground".into())
        );
        assert_eq!(
            world.items().get("sand_cake").unwrap().state(),
            &ItemState::InScene("playground".into())
        );
        assert_eq!(
            world.items().get("milk").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("jam").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("bread").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("raisins").unwrap().state(),
            &ItemState::InScene("kitchen".into())
        );
        assert_eq!(
            world.items().get("ball").unwrap().state(),
            &ItemState::InScene("children_garden".into())
        );
        assert_eq!(
            world.items().get("bad_dog").unwrap().state(),
            &ItemState::InScene("garden".into())
        );
        assert_eq!(
            world.items().get("dumplings").unwrap().state(),
            &ItemState::InScene("children_house".into())
        );
    }

    fn reload_world(world: CakeWorld) -> CakeWorld {
        let mut new_world = CakeWorldBuilder::make_world().unwrap();
        new_world.load(world.dump()).unwrap();
        assert_eq!(new_world.dump(), world.dump());
        new_world
    }

    fn reload_events(
        world: &dyn World,
        narrator: &dyn Narrator,
        events: Vec<Box<dyn Event>>,
    ) -> Vec<Box<dyn Event>> {
        assert!(events.iter().all(|e| e.can_be_triggered(world)));
        let res = events
            .iter()
            .map(|e| {
                narrator
                    .parse_event(world, e.dump())
                    .ok_or_else(|| anyhow::anyhow!("parse_failed"))
            })
            .collect::<Result<Vec<Box<dyn Event>>, _>>()
            .unwrap();
        assert!(res.iter().all(|e| e.can_be_triggered(world)));
        res
    }

    #[test]
    fn workflow() {
        let mut world = prepare_world();
        world = reload_world(world);

        let narrator = narrator::Cake::default();

        // pick sand cake
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|e| e.name() == "pick"));
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        assert!(!events[1].can_be_triggered(&world));
        assert!(!events[1].perform(&mut world));
        world = reload_world(world);

        // give and consume sand cake
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert!(events.iter().all(|e| e.name() == "give_sand_cake"));
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        world = reload_world(world);

        // move both characters to kitchen
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        world = reload_world(world);
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        world = reload_world(world);

        // 6 batches of ingredient pick
        for _ in 1..=6 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            for event in events.iter_mut() {
                if event.get_tags().contains(&"pick".to_string()) {
                    // Put thinkgs to cake
                    let cevent = event
                        .as_any_mut()
                        .downcast_mut::<lib_events::Pick>()
                        .unwrap();
                    if cevent.character() == "kitie" {
                        assert!(cevent.can_be_triggered(&world));
                        assert!(cevent.perform(&mut world));
                        world = reload_world(world);
                    }
                }
            }

            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            for event in events.iter_mut() {
                if event.get_tags().contains(&"use_item".to_string()) {
                    assert!(event.can_be_triggered(&world));
                    assert!(event.perform(&mut world));
                    world = reload_world(world);
                }
            }
        }

        // move both characters to children's garden
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for event in events.iter_mut() {
            if event.get_tags().contains(&"move".to_string()) {
                assert!(event.can_be_triggered(&world));
                assert!(event.perform(&mut world));
                world = reload_world(world);
            }
        }

        // play with children
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        for event in events.iter_mut() {
            let cevent = event
                .as_any_mut()
                .downcast_mut::<lib_events::Pick>()
                .unwrap();
            if cevent.character() == "doggie" {
                assert!(cevent.can_be_triggered(&world));
                assert!(cevent.perform(&mut world));
                world = reload_world(world);
            }
        }

        // move to garden
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for event in events.iter_mut() {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            world = reload_world(world);
        }

        // find big bad dog
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        let mut event = events.remove(0);
        assert!(event.can_be_triggered(&world));
        assert!(event.perform(&mut world));
        world = reload_world(world);

        // go to children house
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for event in narrator.available_events(&world).iter_mut() {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            world = reload_world(world);
        }

        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 8);
        for event in events.iter_mut() {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
            world = reload_world(world);
        }

        // Make sure that doggie and kitie reached final destination
        assert!(world.characters().get("doggie").unwrap().scene() == &Some("way_home".into()));
        assert!(world.characters().get("kitie").unwrap().scene() == &Some("way_home".into()));

        assert!(world.finished());
    }

    #[test]
    fn languages() {
        let mut world = CakeWorldBuilder::make_world().unwrap();
        for lang in vec!["cs", "en"] {
            assert!(world.set_lang(lang));
        }
    }
}
