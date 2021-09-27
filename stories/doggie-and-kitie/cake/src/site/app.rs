use pabitell_lib::{Description, Narrator, World, WorldBuilder};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use yew::prelude::*;

use crate::{narrator, translations, world::CakeWorld, world::CakeWorldBuilder};

use super::{
    action::EventActionItem,
    actions::Actions,
    character_combo::CharacterCombo,
    characters::{self, make_characters},
    message::{Kind as MessageKind, MessageItem},
    messages::{Messages, Msg as MessagesMsg},
    speech::{Msg as SpeechMsg, Speech},
};

pub enum Msg {
    ToggleNavbar,
    UpdateSelectedCharacter(Rc<Option<String>>),
    TriggerEvent(usize),
    TriggerScannedEvent(Value),
    PlayText(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Void,
    QR,
    Use,
    Give,
    Hint,
}

pub struct App {
    world: CakeWorld,
    selected_character: Rc<Option<String>>,
    page: Page,
    navbar_active: bool,
    messages_scope: Rc<RefCell<Option<html::Scope<Messages>>>>,
    speech_scope: Rc<RefCell<Option<html::Scope<Speech>>>>,
}

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("Creating new world");
        let mut world = CakeWorldBuilder::make_world().unwrap();
        world.setup();
        world.set_lang("cs");

        Self {
            world,
            selected_character: Rc::new(None),
            page: Page::Void,
            navbar_active: false,
            messages_scope: Rc::new(RefCell::new(None)),
            speech_scope: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSelectedCharacter(selected_character) => {
                self.selected_character = selected_character;
                if let Some(character) = self.selected_character.as_ref() {
                    let character = self.world.characters().get(character).unwrap();
                    let scene_name = character.scene().as_ref().unwrap();
                    let scene = self.world.scenes().get(scene_name).unwrap();
                    ctx.link()
                        .send_message(Msg::PlayText(scene.long(&self.world)));
                }
                true
            }
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::TriggerEvent(idx) => {
                let narrator = narrator::Cake::default();
                let mut events = narrator.available_events(&self.world);
                let event = &mut events[idx];
                let message = MessageItem::new(
                    translations::get_message("event", self.world.lang(), None),
                    event.success_text(&self.world),
                    MessageKind::Success,
                    Some("fas fa-cogs".to_string()),
                );
                let text = message.text.clone();
                self.messages_scope
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(MessagesMsg::AddMessage(Rc::new(message)));
                let old_screen_text = self.screen_text();
                event.trigger(&mut self.world);
                ctx.link().send_message(Msg::PlayText(text.to_string()));
                if old_screen_text != self.screen_text() {
                    if let Some(text) = self.screen_text() {
                        ctx.link().send_message(Msg::PlayText(text));
                    }
                }
                true
            }
            Msg::TriggerScannedEvent(json_value) => {
                let narrator = narrator::Cake::default();
                if let Some(mut event) = narrator.parse_event(json_value) {
                    let message = if event.can_be_triggered(&self.world) {
                        let old_screen_text = self.screen_text();
                        event.trigger(&mut self.world);
                        if old_screen_text != self.screen_text() {
                            if let Some(text) = self.screen_text() {
                                ctx.link().send_message(Msg::PlayText(text));
                            }
                        }
                        MessageItem::new(
                            translations::get_message("event", self.world.lang(), None),
                            event.success_text(&self.world),
                            MessageKind::Success,
                            Some("fas fa-cogs".to_string()),
                        )
                    } else {
                        MessageItem::new(
                            translations::get_message("event", self.world.lang(), None),
                            event.fail_text(&self.world),
                            MessageKind::Warning,
                            Some("fas fa-cogs".to_string()),
                        )
                    };
                    let text = message.text.clone();
                    self.messages_scope
                        .as_ref()
                        .borrow()
                        .clone()
                        .unwrap()
                        .send_message(MessagesMsg::AddMessage(Rc::new(message)));
                    ctx.link().send_message(Msg::PlayText(text.to_string()));
                    true
                } else {
                    // Can't construct event based on given data
                    // TODO some error message
                    false
                }
            }
            Msg::PlayText(text) => {
                self.speech_scope
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(SpeechMsg::Play(text));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let narrator = narrator::Cake::default();
        let events = narrator.available_events(&self.world);
        let mut characters_map: HashMap<String, Rc<characters::Character>> =
            make_characters(&self.world)
                .into_iter()
                .map(|c| (c.name.to_string(), c))
                .collect();

        let events: Vec<Rc<EventActionItem>> = events
            .into_iter()
            .enumerate()
            .map(|(idx, e)| {
                Rc::new(EventActionItem::new(
                    idx,
                    e.action_text(&self.world),
                    characters_map.get(&e.initiator()).unwrap().clone(),
                    None,
                    None,
                    serde_json::to_vec(&e.dump()).unwrap(),
                ))
            })
            .collect();

        let link = ctx.link();
        let trigger_event_callback = link.callback(|idx| Msg::TriggerEvent(idx));
        let trigger_scanned_event_callback =
            link.callback(|json_value| Msg::TriggerScannedEvent(json_value));

        let lang = self.world.lang().to_string();

        html! {
            <>
                <section class="hero is-small is-light">
                  <div class="hero-body">
                        <p class="title">
                            {self.world.description().short(&self.world)}
                        </p>
                        <p class="subtitle">
                            <Speech
                              lang={lang}
                              start_text={self.world.description().short(&self.world)}
                              shared_scope={self.speech_scope.clone()}
                            />
                        </p>
                    </div>
                </section>
                { self.view_nav(ctx) }
                <main>
                    <Messages shared_scope={ self.messages_scope.clone() }/>
                    { self.view_scene(ctx) }
                    <Actions
                      events={ events }
                      trigger_event={ trigger_event_callback }
                      trigger_scanned_event={ trigger_scanned_event_callback }
                    />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        <a href="https://github.com/shenek/pabitell/"> { "Pabitell" }</a>
                    </div>
                </footer>
            </>
        }
    }
}

impl App {
    fn screen_text(&self) -> Option<String> {
        if let Some(character) = self.selected_character.as_ref() {
            let character = self.world.characters().get(character).unwrap();
            let scene_name = character.scene().as_ref().unwrap();
            let scene = self.world.scenes().get(scene_name).unwrap();
            Some(scene.long(&self.world))
        } else {
            None
        }
    }

    fn view_scene(&self, ctx: &Context<Self>) -> Html {
        if let Some(character) = self.selected_character.as_ref() {
            let character = self.world.characters().get(character).unwrap();
            let scene_name = character.scene().as_ref().unwrap();
            let scene = self.world.scenes().get(scene_name).unwrap();
            html! {
                <section class="section">
                    <h1 class="title">{ scene.short(&self.world) }</h1>
                    <p class="subtitle">
                        <article class="message">
                            <div class="message-body">
                                { scene.long(&self.world) }
                            </div>
                        </article>
                    </p>
                </section>
            }
        } else {
            html! {
                <section class="section">
                </section>
            }
        }
    }

    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let Self { world, .. } = self;
        let link = ctx.link();

        let available_characters = make_characters(world);
        let set_character_callback =
            link.callback(|selected_character| Msg::UpdateSelectedCharacter(selected_character));

        let active_class = if self.navbar_active { "is-active" } else { "" };

        let use_text = translations::get_message("use", world.lang(), None);
        let hint_text = translations::get_message("hint", world.lang(), None);
        let qr_code_text = translations::get_message("qr_code", world.lang(), None);
        let give_text = translations::get_message("give", world.lang(), None);

        html! {
            <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <a class="navbar-item" href="">
                </a>

                <a
                  role="button"
                  class={classes!("navbar-burger", "burger", active_class)}
                  aria-label="menu"
                  aria-expanded="false"
                  data-target="pabitell-navbar"
                  onclick={link.callback(|_| Msg::ToggleNavbar)}
                >
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              <div id="pabitell-navbar" class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                  <a class="navbar-item">{ qr_code_text }</a>
                  <a class="navbar-item">{ use_text }</a>
                  <a class="navbar-item">{ give_text }</a>
                  <a class="navbar-item">{ hint_text }</a>
                </div>

                <div class="navbar-end">
                  <div class="navbar-item">
                    <CharacterCombo
                      available_characters={ available_characters }
                      set_character={ set_character_callback }
                    />
                  </div>
                </div>
              </div>
            </nav>
        }
    }
}
