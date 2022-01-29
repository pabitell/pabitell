pub mod characters;
pub mod events;
pub mod items;
pub mod narrator;
pub mod scenes;
pub mod translations;
pub mod world;

#[cfg(test)]
pub mod tests {
    use pabitell_lib::{Description, Dumpable, Event, ItemState, Narrator, World, WorldBuilder};

    use crate::{
        characters, narrator,
        world::{DollWorld, DollWorldBuilder},
    };

    pub fn prepare_world() -> DollWorld {
        let mut world = DollWorldBuilder::make_world().unwrap();
        world.setup();
        world
    }

    #[test]
    fn setup() {
        let world = prepare_world();
        assert_eq!(
            world.characters().get("kitie").unwrap().scene(),
            &Some("home".into())
        );
        assert_eq!(
            world.characters().get("doggie").unwrap().scene(),
            &Some("home".into())
        );
        assert_eq!(
            world.items().get("doll").unwrap().state(),
            &ItemState::InScene("walk".into())
        );
        assert_eq!(
            world.items().get("small_ball").unwrap().state(),
            &ItemState::InScene("doggie_search".into())
        );
        assert_eq!(
            world.items().get("bucket").unwrap().state(),
            &ItemState::InScene("kitie_search".into())
        );
    }

    fn reload_world(world: DollWorld) -> DollWorld {
        let mut new_world = DollWorldBuilder::make_world().unwrap();
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
                    .parse_event(world, &e.dump())
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

        let narrator = narrator::Doll::default();

        // Talk at home
        for _ in 0..5 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_in_home");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // move characters to walk
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Talk on walk
        for _ in 0..2 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Find doll
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));
        assert!(!events[1].can_be_triggered(&world));
        assert!(!events[1].perform(&mut world));

        // Talk on walk
        for _ in 0..2 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_on_walk");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Move back home
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Talk at home
        for _ in 0..8 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_in_home");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Doggie goes searching
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "move_to_doggie_search");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Doggie finds stuff
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 14);
        for mut event in events {
            assert_eq!(event.name(), "pick");
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Doggie returns searching
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "move_to_home");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Kitie goes searching
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "move_to_kitie_search");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Kitie finds stuff
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 16);
        for mut event in events {
            assert_eq!(event.name(), "pick");
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Kitie returns searching
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "move_to_home");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Give all stuff to doll
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 30);
        for mut event in events {
            assert_eq!(event.name(), "lay_down");
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        assert!(world.finished());
    }

    #[test]
    fn languages() {
        let mut world = DollWorldBuilder::make_world().unwrap();
        for lang in vec!["cs", "en-US"] {
            assert!(world.set_lang(lang));
        }
    }
}
