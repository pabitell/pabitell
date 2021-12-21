use anyhow::{anyhow, Result};
use pabitell_lib::{Narrator, World};
use skim::prelude::*;
use sled::Db;
use uuid::Uuid;

use crate::backend;
#[cfg(feature = "with_doggie_and_kitie_cake")]
use crate::make_story_doggie_and_kitie_cake;
#[cfg(feature = "with_doggie_and_kitie_doll")]
use crate::make_story_doggie_and_kitie_doll;

#[derive(Clone)]
struct PabitellItem {
    code: String,
    short: String,
    long: String,
}

impl SkimItem for PabitellItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.code)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::new_string(self.short.clone(), vec![])
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(self.long.clone())
    }
}

fn select_story(lang: &str) -> Result<Option<PabitellItem>> {
    let mut stories: Vec<PabitellItem> = vec![];

    #[cfg(feature = "with_doggie_and_kitie_cake")]
    {
        let (mut world, _): (Box<dyn World>, Box<dyn Narrator>) =
            make_story_doggie_and_kitie_cake(true)?.unwrap();
        world.set_lang(lang);
        let description = world.description();
        stories.push(PabitellItem {
            code: "doggie_and_kitie_cake".into(),
            short: description.short(world.as_ref()),
            long: description.long(world.as_ref()),
        });
    }

    #[cfg(feature = "with_doggie_and_kitie_doll")]
    {
        let (mut world, _): (Box<dyn World>, Box<dyn Narrator>) =
            make_story_doggie_and_kitie_doll(true)?.unwrap();
        world.set_lang(lang);
        let description = world.description();
        stories.push(PabitellItem {
            code: "doggie_and_kitie_doll".into(),
            short: description.short(world.as_ref()),
            long: description.long(world.as_ref()),
        });
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for story in stories.into_iter() {
        let _ = tx_item.send(Arc::new(story));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .ok_or_else(|| anyhow!("Failed to choose"))?;
    if selected_items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            (*selected_items[0])
                .as_any()
                .downcast_ref::<PabitellItem>()
                .unwrap()
                .clone(),
        ))
    }
}

fn select_language(available_languages: Vec<String>) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for lang in available_languages.into_iter() {
        let _ = tx_item.send(Arc::new(lang));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        (*selected_items[0])
            .as_any()
            .downcast_ref::<String>()?
            .to_string(),
    )
}

#[derive(Clone, Copy)]
enum View {
    MENU,
    ITEMS,
    CHARACTERS,
    SCENES,
    EVENTS,
    CONTROLS,
    RESET,
    LOAD,
    DELETE,
    BACK,
    EXIT,
}

impl SkimItem for View {
    fn text(&self) -> Cow<str> {
        match self {
            Self::MENU => Cow::Borrowed("menu"),
            Self::ITEMS => Cow::Borrowed("items"),
            Self::CHARACTERS => Cow::Borrowed("characters"),
            Self::SCENES => Cow::Borrowed("scenes"),
            Self::EVENTS => Cow::Borrowed("events"),
            Self::CONTROLS => Cow::Borrowed("controls"),
            Self::EXIT => Cow::Borrowed("exit"),
            Self::RESET => Cow::Borrowed("reset"),
            Self::LOAD => Cow::Borrowed("load"),
            Self::DELETE => Cow::Borrowed("delete"),
            Self::BACK => Cow::Borrowed("back"),
        }
    }
}

fn main_menu(world: &dyn World) -> Option<View> {
    println!("{}", world.description().short(world));

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in [
        View::ITEMS,
        View::CHARACTERS,
        View::SCENES,
        View::EVENTS,
        View::CONTROLS,
        View::EXIT,
    ] {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    if selected_items.is_empty() {
        None
    } else {
        Some(
            (*selected_items[0])
                .as_any()
                .downcast_ref::<View>()?
                .clone(),
        )
    }
}

fn select_characters(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let characters: Vec<PabitellItem> = world
        .characters()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world).to_string(),
            long: e.long(world).to_string(),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for character in characters.into_iter() {
        let _ = tx_item.send(Arc::new(character));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| {
                (*e).as_any()
                    .downcast_ref::<PabitellItem>()
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}

fn select_scenes(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let scenes: Vec<PabitellItem> = world
        .scenes()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world).to_string(),
            long: e.long(world).to_string(),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(true) // TODO how does multi work...
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for scene in scenes.into_iter() {
        let _ = tx_item.send(Arc::new(scene));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| {
                (*e).as_any()
                    .downcast_ref::<PabitellItem>()
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}

fn select_items(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let items: Vec<PabitellItem> = world
        .items()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world).to_string(),
            long: e.long(world).to_string(),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(true) // TODO how does multi work...
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in items.into_iter() {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| {
                (*e).as_any()
                    .downcast_ref::<PabitellItem>()
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}

#[derive(Clone)]
struct EventItem {
    idx: usize,
    action: String,
    success: String,
    fail: String,
}

impl SkimItem for EventItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.action)
    }

    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::new_string(self.action.clone(), vec![])
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(self.success.clone())
    }
}

fn select_event(world: &dyn World, narrator: &dyn Narrator) -> Option<Vec<EventItem>> {
    let events = narrator
        .available_events(world)
        .iter()
        .enumerate()
        .map(|(idx, e)| EventItem {
            idx,
            action: e.action_text(world),
            success: e.success_text(world),
            fail: e.fail_text(world),
        })
        .collect::<Vec<EventItem>>();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for event in events.into_iter() {
        let _ = tx_item.send(Arc::new(event));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| (*e).as_any().downcast_ref::<EventItem>().unwrap().clone())
            .collect(),
    )
}

fn controls_menu() -> Option<View> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in [View::RESET, View::LOAD, View::DELETE, View::BACK] {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    if selected_items.is_empty() {
        None
    } else {
        Some(
            (*selected_items[0])
                .as_any()
                .downcast_ref::<View>()?
                .clone(),
        )
    }
}

#[derive(Clone)]
struct WorldItem {
    uuid: String,
}

impl SkimItem for WorldItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.uuid)
    }

    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::new_string(self.uuid.to_string(), vec![])
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(String::new())
    }
}

fn select_stored_world(db: &Db, story: &str) -> Result<Option<Uuid>> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in backend::list_stored(db, story)? {
        let _ = tx_item.send(Arc::new(WorldItem {
            uuid: item.to_string(),
        }));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap();
    if selected_items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            Uuid::parse_str(
                &(*selected_items[0])
                    .as_any()
                    .downcast_ref::<WorldItem>()
                    .unwrap()
                    .uuid,
            )
            .unwrap(),
        ))
    }
}

pub fn start_cli_app(default_lang: &str, db_path: &str) -> Result<()> {
    let story = select_story(default_lang)?.ok_or_else(|| anyhow!("No story picked"))?;
    println!("story: {}", story.short);
    let (mut world, narrator): (Box<dyn World>, Box<dyn Narrator>) = match story.code.as_str() {
        #[cfg(feature = "with_doggie_and_kitie_cake")]
        "doggie_and_kitie_cake" => make_story_doggie_and_kitie_cake(true)?.unwrap(),
        #[cfg(feature = "with_doggie_and_kitie_doll")]
        "doggie_and_kitie_doll" => make_story_doggie_and_kitie_doll(true)?.unwrap(),
        _ => unreachable!(),
    };
    let lang = select_language(
        world
            .available_languages()
            .iter()
            .map(|e| e.to_string())
            .collect(),
    )
    .ok_or_else(|| anyhow!("no language selected"))?;
    println!("lang: {}", lang);

    let mut db = sled::open(db_path).unwrap();

    let mut state = View::MENU;
    let mut selected_characters: Vec<PabitellItem> = vec![];
    let mut selected_items: Vec<PabitellItem> = vec![];
    let mut selected_scenes: Vec<PabitellItem> = vec![];
    loop {
        match state {
            View::MENU => match main_menu(world.as_ref()) {
                Some(View::ITEMS) => state = View::ITEMS,
                Some(View::CHARACTERS) => state = View::CHARACTERS,
                Some(View::SCENES) => state = View::SCENES,
                Some(View::EVENTS) => state = View::EVENTS,
                Some(View::CONTROLS) => state = View::CONTROLS,
                Some(View::EXIT) => break,
                _ => break,
            },
            View::CHARACTERS => {
                if let Some(characters) = select_characters(world.as_ref()) {
                    selected_characters = characters;
                }
                state = View::MENU;
            }
            View::SCENES => {
                if let Some(scenes) = select_scenes(world.as_ref()) {
                    selected_scenes = scenes;
                }
                state = View::MENU;
            }
            View::ITEMS => {
                if let Some(items) = select_items(world.as_ref()) {
                    selected_items = items;
                }
                state = View::MENU;
            }
            View::EVENTS => {
                if !selected_characters.is_empty() {
                    if let Some(scene) = world
                        .characters()
                        .get(&selected_characters[0].code)
                        .ok_or_else(|| {
                            anyhow!(
                                "Failed to found character '{}'",
                                &selected_characters[0].code
                            )
                        })?
                        .scene()
                    {
                        let scene = world
                            .scenes()
                            .get(scene)
                            .ok_or_else(|| anyhow!("Failed to find a scene {}", scene))?;
                        println!("\n{}\n\n", scene.long(world.as_ref()));
                    }
                }
                if let Some(events) = select_event(world.as_mut(), narrator.as_ref()) {
                    if !events.is_empty() {
                        let idx = events[0].idx;
                        let mut events = narrator.available_events(world.as_ref());
                        if events[idx].can_be_triggered(world.as_ref()) {
                            println!("{}", events[idx].success_text(world.as_ref()));
                        } else {
                            println!("{}", events[idx].fail_text(world.as_ref()));
                        }
                        events[idx].trigger(world.as_mut());
                        backend::store(&mut db, &story.code, world.as_ref()).unwrap();
                        continue;
                    }
                }
                state = View::MENU;
            }
            View::CONTROLS => {
                if let Some(new_state) = controls_menu() {
                    state = new_state;
                }
            }
            View::BACK => match state {
                _ => state = View::MENU,
            },
            View::EXIT => break,
            View::RESET => {
                world.clean();
                world.setup();
                state = View::MENU;
            }
            View::DELETE => {
                println!("Deleting world");
                if let Some(uuid) = select_stored_world(&db, &story.code).unwrap() {
                    backend::delete(&mut db, &story.code, &uuid).unwrap();
                    println!("World '{}' was deleted", uuid);
                } else {
                    println!("No world selected");
                }
                state = View::CONTROLS;
            }
            View::LOAD => {
                println!("Loading world");
                if let Some(uuid) = select_stored_world(&db, &story.code).unwrap() {
                    if let Err(error) = backend::load(&mut db, &story.code, &uuid, world.as_mut()) {
                        println!("Failed to load world '{}': {}", uuid, error);
                    } else {
                        println!("World '{}' was loaded", uuid);
                    }
                    state = View::MENU;
                } else {
                    println!("No world selected");
                    state = View::CONTROLS;
                }
            }
        }
        println!(
            "Selected Character: {}",
            selected_characters
                .iter()
                .map(|e| e.short.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "Selected Scenes: {}",
            selected_scenes
                .iter()
                .map(|e| e.short.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "Selected Items: {}",
            selected_items
                .iter()
                .map(|e| e.short.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    Ok(())
}
