use gloo::storage::{self, Storage};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{OrientationLockType, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

use crate::{
    events, translations,
    webapp::{
        action_event::ActionEventItem,
        actions::{Actions, Msg as ActionsMsg},
        character_switch::CharacterSwitch,
        characters,
        intro::Intro,
        items::Item,
        message::{Kind as MessageKind, MessageItem},
        messages::{Messages, Msg as MessagesMsg},
        print::{Print, PrintItem},
        speech::{Msg as SpeechMsg, Speech},
        status::{Msg as StatusMsg, Status},
    },
    Narrator, World,
};

pub enum Msg {
    UpdateSelectedCharacter(Rc<Option<String>>),
    TriggerEvent(usize),
    TriggerScannedEvent(Value),
    TriggerScannedCharacter(Option<String>, Uuid),
    PlayText(String),
    NotificationRecieved(String),
    Reset,
    CreateNewWorld,
    NewWorldIdFetched(Uuid),
    WorldUpdateFetched(Box<dyn World>),
    RefreshWorld,
    StatusReady,
    ShowPrint(bool),
    Void(bool),
    ScreenOrientationLocked(Option<JsValue>),
}

pub struct App {
    world_id: Option<Uuid>,
    world: Option<Box<dyn World>>,
    selected_character: Rc<Option<String>>,
    messages_scope: Rc<RefCell<Option<html::Scope<Messages>>>>,
    speech_scope: Rc<RefCell<Option<html::Scope<Speech>>>>,
    status_scope: Rc<RefCell<Option<html::Scope<Status>>>>,
    actions_scope: Rc<RefCell<Option<html::Scope<Actions>>>>,
    ws_queue: Vec<String>,
    loading: Rc<RefCell<bool>>,
    show_print: bool,
    orientation_lock: Option<JsValue>,
    event_count: usize,
}

async fn make_request(
    url: &str,
    method: &str,
    data: Option<Value>,
) -> Result<(Option<Value>, u16), JsValue> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    opts.body(data.map(|e| JsValue::from_str(&e.to_string())).as_ref());

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    log::debug!("{} ({})", url, resp.status());

    let res = JsFuture::from(resp.json()?).await?;
    Ok((res.into_serde().ok(), resp.status()))
}

#[derive(Properties)]
pub struct Props {
    pub make_characters: Option<Box<dyn Fn(&dyn World) -> Rc<Vec<Rc<characters::Character>>>>>,
    pub make_narrator: Option<Box<dyn Fn() -> Box<dyn Narrator>>>,
    pub make_print_items: Option<Box<dyn Fn(Box<dyn World>) -> Vec<PrintItem>>>,
    pub make_owned_items: Option<Box<dyn Fn(&dyn World, &Option<String>) -> Rc<Vec<Rc<Item>>>>>,
    pub make_world: Option<Box<dyn Fn(&str) -> Box<dyn World>>>,
    pub story_name: String,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Default for Props {
    fn default() -> Self {
        Self {
            make_characters: None,
            make_narrator: None,
            make_world: None,
            make_owned_items: None,
            make_print_items: None,
            story_name: String::default(),
        }
    }
}

impl Clone for Props {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let world_id = if let Ok(world_id) = storage::LocalStorage::get("world_id") {
            let world_id: String = world_id;
            Uuid::parse_str(&world_id).ok()
        } else {
            None
        };

        // Screen orientation locking
        let screen_orientation = web_sys::window().unwrap().screen().unwrap().orientation();
        ctx.link().send_future(async move {
            let res = if let Ok(promise) = screen_orientation.lock(OrientationLockType::Any) {
                if let Ok(value) = JsFuture::from(promise).await {
                    Some(value)
                } else {
                    None
                }
            } else {
                None
            };
            Msg::ScreenOrientationLocked(res)
        });

        let mut res = Self {
            world_id,
            world: None,
            selected_character: Rc::new(storage::LocalStorage::get("fixed_character").ok()),
            messages_scope: Rc::new(RefCell::new(None)),
            speech_scope: Rc::new(RefCell::new(None)),
            status_scope: Rc::new(RefCell::new(None)),
            actions_scope: Rc::new(RefCell::new(None)),
            ws_queue: vec![],
            loading: Rc::new(RefCell::new(false)),
            show_print: false,
            orientation_lock: None,
            event_count: 0,
        };

        if let Some(world_id) = world_id.as_ref() {
            log::info!("World Id: {:?}", &world_id);
            // Update data from the server
            res.request_to_get_world(ctx, *world_id);
        }

        res
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSelectedCharacter(selected_character) => {
                self.selected_character = selected_character;
                if let Some(world) = &self.world {
                    if let Some(character) = self.selected_character.as_ref() {
                        let character = world.characters().get(character).unwrap();
                        let scene_name = character.scene().as_ref().unwrap();
                        let scene = world.scenes().get(scene_name).unwrap();
                        ctx.link()
                            .send_message(Msg::PlayText(scene.long(world.as_ref())));
                    }
                    true
                } else {
                    false
                }
            }
            Msg::TriggerEvent(idx) => {
                if let Some(world) = self.world.as_mut() {
                    let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                    let mut events = narrator.available_events(world.as_ref());
                    let event = &mut events[idx];
                    self.request_to_trigger_event(
                        ctx,
                        self.world_id.unwrap().clone(),
                        event.dump(),
                    );
                    true
                } else {
                    false
                }
            }
            Msg::TriggerScannedEvent(json_value) => {
                let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                if let Some(world) = self.world.as_mut() {
                    if let Some(mut event) = narrator.parse_event(world.as_ref(), &json_value) {
                        // Update initiator
                        if let Some(character) = self.selected_character.clone().as_ref() {
                            event.set_initiator(character.to_string());
                        }

                        // Render events which can be triggered immediately
                        if !event.can_be_triggered(world.as_ref()) {
                            let message = MessageItem::new(
                                translations::get_message_global("event", world.lang(), None),
                                event.fail_text(world.as_ref()),
                                MessageKind::Warning,
                                Some("fas fa-cogs".to_string()),
                            );

                            self.messages_scope
                                .as_ref()
                                .borrow()
                                .clone()
                                .unwrap()
                                .send_message(MessagesMsg::AddMessage(Rc::new(message)));
                        }

                        self.request_to_trigger_event(
                            ctx,
                            self.world_id.unwrap().clone(),
                            event.dump(),
                        );
                    } else {
                        // Can't construct event based on given data
                        // TODO some error message
                        log::warn!("Failed to parse event from {}", json_value.to_string());
                        return false;
                    }
                } else {
                    return false;
                }
                false
            }
            Msg::TriggerScannedCharacter(character, world_id) => {
                // check whether character exists in the world
                let world = ctx.props().make_world.as_ref().unwrap()("cs");
                if let Some(character) = character {
                    if let Some(character_instance) = world.characters().get(&character) {
                        storage::LocalStorage::set("world_id", world_id).unwrap();
                        storage::LocalStorage::set("fixed_character", &character).unwrap();
                        self.world_id = Some(world_id);

                        // Set the character
                        ctx.link()
                            .send_message(Msg::UpdateSelectedCharacter(Rc::new(Some(character))));
                        // Get the world
                        self.request_to_get_world(ctx, world_id);

                        // Queue character notification
                        // we need to wait till Status component is initialized
                        self.ws_queue.push(character_instance.dump().to_string());

                        true
                    } else {
                        log::warn!("Character '{}' is not found", character);
                        // TODO display message that character is not found
                        false
                    }
                } else {
                    self.world_id = Some(world_id);
                    ctx.link()
                        .send_message(Msg::UpdateSelectedCharacter(Rc::new(None)));

                    // Get the world
                    self.request_to_get_world(ctx, world_id);

                    true
                }
            }
            Msg::PlayText(text) => {
                if let Some(speech) = self.speech_scope.as_ref().borrow().clone() {
                    log::debug!("Playing: {}", &text);
                    speech.send_message(SpeechMsg::Play(text));
                } else {
                    log::warn!("Speech not initialized");
                }
                false
            }
            Msg::Reset => {
                self.world = None;
                storage::LocalStorage::delete("world_id");
                storage::LocalStorage::delete("fixed_character");
                if let Some(scope) = self.messages_scope.as_ref().borrow().clone() {
                    scope.send_message(MessagesMsg::Clear);
                }
                true
            }
            Msg::NewWorldIdFetched(world_id) => {
                log::info!("New world id fetched {}", &world_id);
                storage::LocalStorage::set("world_id", world_id).unwrap();
                self.world_id = Some(world_id);
                self.request_to_get_world(ctx, world_id);
                true
            }
            Msg::WorldUpdateFetched(world) => {
                let old_screen_text = App::screen_text(&self.world, &self.selected_character);
                self.event_count = world.event_count();
                self.world = Some(world);
                let new_screen_text = App::screen_text(&self.world, &self.selected_character);
                if new_screen_text != old_screen_text {
                    if let Some(text) = new_screen_text {
                        ctx.link().send_message(Msg::PlayText(text));
                    }
                }
                true
            }
            Msg::CreateNewWorld => {
                self.request_to_create_new_world(ctx);
                true
            }
            Msg::Void(render) => render,
            Msg::RefreshWorld => {
                if let Some(world_id) = self.world_id.clone() {
                    self.request_to_get_world(ctx, world_id);
                }
                true
            }
            Msg::NotificationRecieved(data) => {
                let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                if let Ok(json) = serde_json::from_str(&data) {
                    let json: Value = json;
                    let world = if let Some(world) = self.world.as_ref() {
                        world
                    } else {
                        return false;
                    };
                    // Now we should determine which type of notification this really is
                    if let Some(event) = narrator.parse_event(world.as_ref(), &json["event"]) {
                        if let Value::Number(number) = &json["event_count"] {
                            if let Some(count) = number.as_u64() {
                                self.event_count = count as usize;
                            } else {
                                log::warn!("Event notification is not positive integer");
                                return false;
                            }
                        } else {
                            log::warn!("Malformed event notification count");
                            return false;
                        }
                        log::info!("New event arrived from ws");
                        if let Some(actions_scope) = self.actions_scope.as_ref().borrow().as_ref() {
                            log::debug!("Hiding QR code of actions");
                            actions_scope.send_message(ActionsMsg::QRCodeHide);
                        }
                        if let Some(world_id) = self.world_id {
                            // TODO this should be optimized
                            // not need to refresh the whole world just a single event
                            self.request_to_get_world(ctx, world_id);
                        }

                        if let Some(world) = self.world.as_ref() {
                            if event.can_be_triggered(world.as_ref()) {
                                let message = MessageItem::new(
                                    translations::get_message_global("event", world.lang(), None),
                                    event.success_text(world.as_ref()),
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
                                ctx.link().send_message(Msg::PlayText(text.to_string()));
                            }
                        }
                    } else if let Value::String(name) = &json["name"] {
                        // It can be a message that character was joining the game
                        // => close Modal which is displaying QR code
                        log::info!("Character '{}' joined", name);
                        if let Some(actions_scope) = self.actions_scope.as_ref().borrow().as_ref() {
                            actions_scope.send_message(ActionsMsg::QRCodeHide);
                        }
                    }
                }
                false
            }
            Msg::StatusReady => {
                // Send notification to other connected clients
                // Note that notification needs to be send once
                // status_scope in initialized by the subcomponent
                let status_scope = self.status_scope.clone();

                self.ws_queue.drain(..).for_each(|msg| {
                    let status_scope = status_scope.clone();
                    ctx.link().send_future(async move {
                        if let Some(status_scope) = status_scope.as_ref().borrow().as_ref() {
                            status_scope.send_message(StatusMsg::SendMessage(msg));
                        }
                        Msg::Void(false)
                    });
                });
                false
            }
            Msg::ShowPrint(show) => {
                if self.show_print != show {
                    self.show_print = show;
                    true
                } else {
                    false
                }
            }
            Msg::ScreenOrientationLocked(value) => {
                self.orientation_lock = value;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let loading_classes = if *self.loading.borrow() {
            classes!("modal", "is-active")
        } else {
            classes!("modal")
        };
        let loading = html! {
                <div class={loading_classes}>
                    <div class="modal-background"></div>
                    <div class="modal-content has-text-centered">
                        <figure class="image is-square is-64x64 inverted is-inline-block rotate">
                            <img src="images/spinner.svg" alt="" />
                        </figure>
                    </div>
                </div>
        };

        if self.show_print {
            let close_cb = link.callback(|_| Msg::ShowPrint(false));
            return html! {
                <Print {close_cb} items={
                    ctx.props().make_print_items.as_ref().unwrap()(
                        ctx.props().make_world.as_ref().unwrap()("cs")
                    )
                  }
                />
            };
        }

        if let Some(world) = &self.world {
            let available_characters =
                ctx.props().make_characters.as_ref().unwrap()(world.as_ref());
            let set_character_callback = link
                .callback(|selected_character| Msg::UpdateSelectedCharacter(selected_character));
            let narrator = ctx.props().make_narrator.as_ref().unwrap()();
            let events = narrator.available_events(world.as_ref());
            let characters_map: HashMap<String, Rc<characters::Character>> =
                ctx.props().make_characters.as_ref().unwrap()(world.as_ref())
                    .iter()
                    .map(|c| (c.name.to_string(), c.clone()))
                    .collect();

            let events: Vec<Rc<ActionEventItem>> = events
                .into_iter()
                .enumerate()
                .map(|(idx, e)| {
                    let (image, self_triggering) =
                        if let Some(event) = e.as_any().downcast_ref::<events::Pick>() {
                            (Some(format!("images/{}.svg", event.item())), false)
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Give>() {
                            (Some(format!("images/{}.svg", event.item())), false)
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Move>() {
                            (Some(format!("images/{}.svg", event.scene())), false)
                        } else if let Some(event) = e.as_any().downcast_ref::<events::UseItem>() {
                            (Some(format!("images/{}.svg", event.item())), false)
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Void>() {
                            (
                                event.item().as_ref().map(|e| format!("images/{}.svg", e)),
                                false,
                            )
                        } else if e.as_any().downcast_ref::<events::Talk>().is_some() {
                            (Some("images/talk.svg".to_owned()), true)
                        } else {
                            (None, false)
                        };
                    Rc::new(ActionEventItem::new(
                        idx,
                        e.action_text(world.as_ref()),
                        characters_map.get(&e.initiator()).unwrap().clone(),
                        None,
                        image,
                        serde_json::to_vec(&e.dump()).unwrap(),
                        self_triggering,
                    ))
                })
                .collect();

            let trigger_event_callback = link.callback(|idx| Msg::TriggerEvent(idx));
            let trigger_scanned_event_callback =
                link.callback(|json_value| Msg::TriggerScannedEvent(json_value));

            let lang = world.lang().to_string();

            let notification_cb = link.callback(|data| Msg::NotificationRecieved(data));
            let status_ready_cb = link.callback(|_| Msg::StatusReady);
            let reset_cb = link.callback(|_| Msg::Reset);
            let refresh_world_cb = link.callback(|_| Msg::RefreshWorld);

            let finished = if let Some(world) = self.world.as_ref() {
                world.finished()
            } else {
                false
            };

            let fixed_character: Option<String> =
                storage::LocalStorage::get("fixed_character").ok();

            html! {
                <>
                    <section class="hero is-small is-light">
                      <div class="hero-body">
                          <p class="title">
                            {world.description().short(world.as_ref())}
                          </p>
                          <div class="subtitle is-flex">
                              <div class="w-100 field is-grouped is-grouped-multiline is-justify-content-center">
                                  <div class="has-text-centered">
                                      <Speech
                                        lang={lang.clone()}
                                        start_text={world.description().short(world.as_ref())}
                                        shared_scope={self.speech_scope.clone()}
                                        world_name={world.name()}
                                      />
                                  </div>
                              </div>
                          </div>
                          <div class="subtitle is-flex">
                              <div class="w-100 field is-grouped is-grouped-multiline is-justify-content-center">
                                  <div class="has-text-centered">
                                      <Status
                                        world_id={self.world_id.clone()}
                                        namespace={"some_namespace"}
                                        story={ctx.props().story_name.clone()}
                                        msg_recieved={notification_cb}
                                        status_ready={status_ready_cb}
                                        refresh_world={refresh_world_cb}
                                        reset_world={reset_cb}
                                        status_scope={self.status_scope.clone()}
                                        event_count={self.event_count}
                                      />
                                  </div>
                              </div>
                          </div>
                      </div>
                    </section>
                    <main>
                        <CharacterSwitch
                          available_characters={ available_characters.clone() }
                          set_character={ set_character_callback }
                          selected_character={ self.selected_character.clone() }
                          fixed={fixed_character.is_some()}
                        />
                        <Actions
                          lang={ lang }
                          available_characters={ available_characters }
                          owned_items={ ctx.props().make_owned_items.as_ref().unwrap()(world.as_ref(), self.selected_character.as_ref()) }
                          selected_character={ self.selected_character.clone() }
                          events={ events }
                          trigger_event={ trigger_event_callback }
                          trigger_scanned_event={ trigger_scanned_event_callback }
                          world_id={self.world_id.unwrap_or_default().clone()}
                          actions_scope={self.actions_scope.clone()}
                          { finished }
                        />
                        { self.view_scene(ctx) }
                        <Messages shared_scope={ self.messages_scope.clone() }/>
                    </main>
                    <footer class="footer">
                        <div class="content has-text-centered">
                            <a href="https://github.com/shenek/pabitell/"> { "Pabitell" }</a>
                        </div>
                    </footer>
                    { loading }
                </>
            }
        } else {
            let new_world_cb = link.callback(|_| Msg::CreateNewWorld);
            let character_scanned_cb = link.callback(|(character, world_id)| {
                Msg::TriggerScannedCharacter(character, world_id)
            });
            let show_print_cb = link.callback(|show| Msg::ShowPrint(show));

            let world = ctx.props().make_world.as_ref().unwrap()("cs");

            html! {
                <>
                    <Intro
                        new_world={new_world_cb}
                        story_name={world.description().short(world.as_ref())}
                        story_detail={world.description().long(world.as_ref())}
                        character_scanned={character_scanned_cb}
                        show_print={show_print_cb}
                    />
                    { loading }
                </>
            }
        }
    }
}

impl App {
    fn base_url(ctx: &Context<Self>) -> String {
        format!("/api/some_namespace/{}/", ctx.props().story_name)
    }

    fn screen_text(
        world: &Option<Box<dyn World>>,
        selected_character: &Option<String>,
    ) -> Option<String> {
        let world = world.as_ref()?;
        if let Some(character) = selected_character.as_ref() {
            let character = world.characters().get(character).unwrap();
            let scene_name = character.scene().as_ref().unwrap();
            let scene = world.scenes().get(scene_name).unwrap();
            Some(scene.long(world.as_ref()))
        } else {
            None
        }
    }

    fn view_scene(&self, ctx: &Context<Self>) -> Html {
        if let Some(world) = &self.world {
            let onclick = ctx.link().callback(|_| Msg::Reset);
            let restart_text = translations::get_message_global("restart", world.lang(), None);
            let end_text = translations::get_message_global("end", world.lang(), None);
            let restart = html! {
                <article class="message is-info">
                    <div class="message-header">
                        <span class="icon-text">
                          <span class="icon">
                            <i class="fas fa-thumbs-up"></i>
                          </span>
                          <span>{ end_text }</span>
                        </span>
                    </div>
                    <div class="message-body">
                        <div class="buttons">
                            <button {onclick} class="button is-outlined is-info">
                                <span class="icon"><i class="fas fa-redo-alt"></i></span>
                                <span>{ restart_text }</span>
                            </button>
                        </div>
                    </div>
                </article>
            };

            let scene_description = if let Some(character) = self.selected_character.as_ref() {
                let character = world.characters().get(character).unwrap();
                let scene_name = character.scene().as_ref().unwrap();
                let scene = world.scenes().get(scene_name).unwrap();

                html! {
                    <>
                        <h1 class="title">{ scene.short(world.as_ref()) }</h1>
                        <p class="subtitle">
                            <article class="message">
                                <div class="message-body">
                                    { scene.long(world.as_ref()) }
                                </div>
                            </article>
                        </p>
                    </>
                }
            } else {
                html! {}
            };

            let class = if world.finished() || self.selected_character.is_some() {
                classes!("section")
            } else {
                classes!("section", "is-hidden")
            };

            html! {
                <section {class}>
                { scene_description }
                { if world.finished() { restart } else { html! {} } }
                </section>
            }
        } else {
            html! {}
        }
    }

    fn request_to_create_new_world(&mut self, ctx: &Context<Self>) {
        *self.loading.borrow_mut() = true;
        let loading = self.loading.clone();
        let url = Self::base_url(ctx);
        ctx.link().send_future(async move {
            let res = make_request(&url, "POST", None).await;
            *loading.borrow_mut() = false;
            match res {
                Ok((Some(json), status)) => {
                    if let Some(Value::String(id_str)) = json.get("id") {
                        if let Ok(uuid) = Uuid::parse_str(id_str) {
                            log::info!("New World Id: {:?}", &uuid);
                            Msg::NewWorldIdFetched(uuid)
                        } else {
                            Msg::Void(true)
                        }
                    } else {
                        Msg::Void(true)
                    }
                }
                Ok((None, status)) => {
                    log::debug!("{} ({})", url, status);
                    log::warn!(
                        "No JSON response from the server when creating new world (http={})",
                        status
                    );
                    Msg::Void(true)
                }
                Err(e) => {
                    log::warn!("Failed to fetch: {:?}", e);
                    Msg::Void(true)
                }
            }
        });
    }

    fn request_to_get_world(&mut self, ctx: &Context<Self>, world_id: Uuid) {
        *self.loading.borrow_mut() = true;
        let loading = self.loading.clone();
        let mut world = ctx.props().make_world.as_ref().unwrap()("cs");
        let url = Self::base_url(ctx);
        ctx.link().send_future(async move {
            let url = format!("{}{}/", url, world_id);
            let res = make_request(&url, "GET", None).await;
            *loading.borrow_mut() = false;
            match res {
                Ok((Some(json), status)) => match status {
                    404 => {
                        log::warn!("World with id {} was not found on the server", world_id);
                        Msg::Reset
                    }
                    e if 200 <= e && e < 300 => {
                        if let Err(e) = world.load(json) {
                            log::warn!("Fetched world in wrong format: {:?}", e);
                            Msg::Void(true)
                        } else {
                            log::debug!("World {} updated", world_id);
                            Msg::WorldUpdateFetched(world)
                        }
                    }
                    _ => {
                        log::warn!("Wrong server response when downloading world {}", world_id);
                        Msg::Void(true)
                    }
                },
                Ok((None, _)) => {
                    log::warn!("No JSON response from the server when creating new world");
                    Msg::Void(true)
                }
                Err(e) => {
                    log::warn!("Failed to fetch: {:?}", e);
                    Msg::Void(true)
                }
            }
        })
    }

    fn request_to_trigger_event(&mut self, ctx: &Context<Self>, world_id: Uuid, event_json: Value) {
        *self.loading.borrow_mut() = true;
        let loading = self.loading.clone();
        let url = Self::base_url(ctx);
        ctx.link().send_future(async move {
            let url = format!("{}{}/event/", url, world_id);
            let res = make_request(&url, "POST", Some(event_json)).await;
            *loading.borrow_mut() = false;
            match res {
                Ok((resp, status)) => {
                    match status {
                        e if 200 <= e && e < 300 => {
                            log::debug!("Event triggered {:?}", resp);
                            Msg::RefreshWorld
                        }
                        _ => {
                            // TODO failed to publish event message
                            Msg::Void(true)
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to trigger: {:?}", e);
                    Msg::Void(true)
                }
            }
        })
    }
}
