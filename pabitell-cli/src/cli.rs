use anyhow::{anyhow, Result};
use pabitell_lib::{Narrator, World};
use skim::prelude::*;
use sled::Db;
use std::io::prelude::*;
use term::color::{self, Color};
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

fn println<S>(text_color: Color, text: S)
where
    S: std::fmt::Display,
{
    let mut t = term::stdout().unwrap();
    t.fg(text_color).unwrap();
    writeln!(t, "{text}").unwrap();
    t.reset().unwrap();
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
    Menu,
    Items,
    Characters,
    Scenes,
    Events,
    Controls,
    Reset,
    Load,
    Delete,
    Back,
    Exit,
}

impl SkimItem for View {
    fn text(&self) -> Cow<str> {
        match self {
            Self::Menu => Cow::Borrowed("menu"),
            Self::Items => Cow::Borrowed("items"),
            Self::Characters => Cow::Borrowed("characters"),
            Self::Scenes => Cow::Borrowed("scenes"),
            Self::Events => Cow::Borrowed("events"),
            Self::Controls => Cow::Borrowed("controls"),
            Self::Exit => Cow::Borrowed("exit"),
            Self::Reset => Cow::Borrowed("reset"),
            Self::Load => Cow::Borrowed("load"),
            Self::Delete => Cow::Borrowed("delete"),
            Self::Back => Cow::Borrowed("back"),
        }
    }
}

fn main_menu(world: &dyn World) -> Option<View> {
    println(color::BRIGHT_BLUE, world.description().short(world));

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in [
        View::Items,
        View::Characters,
        View::Scenes,
        View::Events,
        View::Controls,
        View::Exit,
    ] {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    if selected_items.is_empty() {
        None
    } else {
        Some(*(*selected_items[0]).as_any().downcast_ref::<View>()?)
    }
}

fn select_characters(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let characters: Vec<PabitellItem> = world
        .characters()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world),
            long: e.long(world),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for character in characters {
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
            short: e.short(world),
            long: e.long(world),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(true) // TODO how does multi work...
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for scene in scenes {
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
    let items = world.items().values().map(|e| PabitellItem {
        code: e.name().to_string(),
        short: e.short(world),
        long: e.long(world),
    });

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(true) // TODO how does multi work...
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in items {
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
    #[allow(dead_code)]
    fail: String,
}

impl SkimItem for EventItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.action)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::new_string(self.action.clone(), vec![])
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(self.success.clone())
    }
}

fn select_event(world: &dyn World, narrator: &dyn Narrator) -> Option<Vec<EventItem>> {
    let events = narrator
        .available_events_sorted(world)
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
    for event in events {
        let _ = tx_item.send(Arc::new(event.to_owned()));
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
    for item in [View::Reset, View::Load, View::Delete, View::Back] {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    if selected_items.is_empty() {
        None
    } else {
        Some(*(*selected_items[0]).as_any().downcast_ref::<View>()?)
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

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
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
    println(color::BRIGHT_MAGENTA, format!("story: {}", story.short));
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
    println(color::BRIGHT_MAGENTA, format!("lang: {lang}"));

    let mut db = sled::open(db_path).unwrap();

    let mut state = View::Menu;
    let mut selected_characters: Vec<PabitellItem> = vec![];
    let mut selected_items: Vec<PabitellItem> = vec![];
    let mut selected_scenes: Vec<PabitellItem> = vec![];
    loop {
        match state {
            View::Menu => match main_menu(world.as_ref()) {
                Some(View::Items) => state = View::Items,
                Some(View::Characters) => state = View::Characters,
                Some(View::Scenes) => state = View::Scenes,
                Some(View::Events) => state = View::Events,
                Some(View::Controls) => state = View::Controls,
                Some(View::Exit) => break,
                _ => break,
            },
            View::Characters => {
                if let Some(characters) = select_characters(world.as_ref()) {
                    selected_characters = characters;
                }
                state = View::Menu;
            }
            View::Scenes => {
                if let Some(scenes) = select_scenes(world.as_ref()) {
                    selected_scenes = scenes;
                }
                state = View::Menu;
            }
            View::Items => {
                if let Some(items) = select_items(world.as_ref()) {
                    selected_items = items;
                }
                state = View::Menu;
            }
            View::Events => {
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
                        println(
                            color::BRIGHT_GREEN,
                            format!("\n{}\n\n", scene.long(world.as_ref())),
                        );
                    }
                }
                if let Some(events) = select_event(world.as_mut(), narrator.as_ref()) {
                    if !events.is_empty() {
                        let idx = events[0].idx;
                        let mut events = narrator.available_events_sorted(world.as_ref());
                        if events[idx].can_be_triggered(world.as_ref()) {
                            println(
                                color::BRIGHT_CYAN,
                                format!(
                                    "{}. {}",
                                    world.event_count() + 1,
                                    events[idx].success_text(world.as_ref())
                                ),
                            );
                        } else {
                            println(color::BRIGHT_RED, events[idx].fail_text(world.as_ref()));
                        }
                        events[idx].trigger(world.as_mut());
                        backend::store(&mut db, &story.code, world.as_ref()).unwrap();
                        continue;
                    }
                }
                state = View::Menu;
            }
            View::Controls => {
                if let Some(new_state) = controls_menu() {
                    state = new_state;
                }
            }
            View::Back => {
                state = View::Menu;
            }
            View::Exit => break,
            View::Reset => {
                world.clean();
                world.setup();
                state = View::Menu;
            }
            View::Delete => {
                println(color::BRIGHT_MAGENTA, "Deleting world");
                if let Some(uuid) = select_stored_world(&db, &story.code).unwrap() {
                    backend::delete(&mut db, &story.code, &uuid).unwrap();
                    println(
                        color::BRIGHT_MAGENTA,
                        format!("World '{}' was deleted", uuid),
                    );
                } else {
                    println(color::BRIGHT_MAGENTA, "No world selected");
                }
                state = View::Controls;
            }
            View::Load => {
                println(color::BRIGHT_MAGENTA, "Loading world");
                if let Some(uuid) = select_stored_world(&db, &story.code).unwrap() {
                    if let Err(error) = backend::load(&db, &story.code, &uuid, world.as_mut()) {
                        println(
                            color::BRIGHT_RED,
                            format!("Failed to load world '{}': {}", uuid, error),
                        );
                    } else {
                        println(
                            color::BRIGHT_MAGENTA,
                            format!("World '{}' was loaded", uuid),
                        );
                    }
                    state = View::Menu;
                } else {
                    println(color::BRIGHT_MAGENTA, "No world selected");
                    state = View::Controls;
                }
            }
        }
        println(
            color::BRIGHT_MAGENTA,
            format!(
                "Selected Character: {}",
                selected_characters
                    .iter()
                    .map(|e| e.short.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        );
        println(
            color::BRIGHT_MAGENTA,
            format!(
                "Selected Scenes: {}",
                selected_scenes
                    .iter()
                    .map(|e| e.short.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        );
        println(
            color::BRIGHT_MAGENTA,
            format!(
                "Selected Items: {}",
                selected_items
                    .iter()
                    .map(|e| e.short.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        );
    }

    Ok(())
}
