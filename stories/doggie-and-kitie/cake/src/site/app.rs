use gloo::storage::{self, Storage};
use pabitell_lib::{Description, Dumpable, Id, Named, Narrator, World, WorldBuilder};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

use crate::{narrator, translations, world::CakeWorld, world::CakeWorldBuilder};

use super::{
    action_event::ActionEventItem,
    actions::{Actions, Msg as ActionsMsg},
    character_switch::CharacterSwitch,
    characters::{self, make_characters},
    intro::Intro,
    items::{self, make_owned_items},
    message::{Kind as MessageKind, MessageItem},
    messages::{Messages, Msg as MessagesMsg},
    print::Print,
    print_items::make_print_items,
    speech::{Msg as SpeechMsg, Speech},
    status::{Msg as StatusMsg, Status},
};

pub enum Msg {
    UpdateSelectedCharacter(Rc<Option<String>>),
    TriggerEvent(usize),
    TriggerScannedEvent(Value),
    TriggerScannedCharacter(String, Uuid),
    PlayText(String),
    NotificationRecieved(String),
    Reset,
    CreateNewWorld,
    NewWorldIdFetched(Uuid),
    WorldUpdateFetched(CakeWorld),
    EventPublished,
    StatusReady,
    ShowPrint(bool),
    Void(bool),
}

pub struct App {
    world_id: Option<Uuid>,
    world: Option<CakeWorld>,
    selected_character: Rc<Option<String>>,
    messages_scope: Rc<RefCell<Option<html::Scope<Messages>>>>,
    speech_scope: Rc<RefCell<Option<html::Scope<Speech>>>>,
    status_scope: Rc<RefCell<Option<html::Scope<Status>>>>,
    actions_scope: Rc<RefCell<Option<html::Scope<Actions>>>>,
    ws_queue: Vec<String>,
    loading: Rc<RefCell<bool>>,
    show_print: bool,
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

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {}

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

        let mut res = Self {
            world_id,
            world: None,
            selected_character: Rc::new(None),
            messages_scope: Rc::new(RefCell::new(None)),
            speech_scope: Rc::new(RefCell::new(None)),
            status_scope: Rc::new(RefCell::new(None)),
            actions_scope: Rc::new(RefCell::new(None)),
            ws_queue: vec![],
            loading: Rc::new(RefCell::new(false)),
            show_print: false,
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
                        ctx.link().send_message(Msg::PlayText(scene.long(world)));
                    }
                    true
                } else {
                    false
                }
            }
            Msg::TriggerEvent(idx) => {
                let old_screen_text = self.screen_text();
                if let Some(world) = self.world.as_mut() {
                    let narrator = narrator::Cake::default();
                    let mut events = narrator.available_events(world);
                    let event = &mut events[idx];
                    let message = MessageItem::new(
                        translations::get_message("event", world.lang(), None),
                        event.success_text(world),
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
                    self.request_to_trigger_event(
                        ctx,
                        self.world_id.unwrap().clone(),
                        event.dump(),
                    );
                    ctx.link().send_message(Msg::PlayText(text.to_string()));
                    if old_screen_text != self.screen_text() {
                        if let Some(text) = self.screen_text() {
                            ctx.link().send_message(Msg::PlayText(text));
                        }
                    }
                    true
                } else {
                    false
                }
            }
            Msg::TriggerScannedEvent(json_value) => {
                let narrator = narrator::Cake::default();
                let old_screen_text = self.screen_text();
                if let Some(world) = self.world.as_mut() {
                    if let Some(mut event) = narrator.parse_event(json_value) {
                        // Update initiator
                        if let Some(character) = self.selected_character.clone().as_ref() {
                            event.set_initiator(character.to_string());
                        }

                        let message = if event.can_be_triggered(world) {
                            MessageItem::new(
                                translations::get_message("event", world.lang(), None),
                                event.success_text(world),
                                MessageKind::Success,
                                Some("fas fa-cogs".to_string()),
                            )
                        } else {
                            MessageItem::new(
                                translations::get_message("event", world.lang(), None),
                                event.fail_text(world),
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
                        self.request_to_trigger_event(
                            ctx,
                            self.world_id.unwrap().clone(),
                            event.dump(),
                        );
                    } else {
                        // Can't construct event based on given data
                        // TODO some error message
                        return false;
                    }
                } else {
                    return false;
                }
                let new_screen_text = self.screen_text();
                if old_screen_text != new_screen_text {
                    if let Some(text) = new_screen_text {
                        ctx.link().send_message(Msg::PlayText(text));
                    }
                }
                false
            }
            Msg::TriggerScannedCharacter(character, world_id) => {
                // check whether character exists in the world
                let world = Self::make_world();
                if let Some(character_instance) = world.characters().get(&character) {
                    storage::LocalStorage::set("world_id", world_id).unwrap();
                    storage::LocalStorage::set("fixed_character", &character).unwrap();
                    self.world_id = Some(world_id);

                    // Setch the character
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
                self.world = Some(world);
                true
            }
            Msg::CreateNewWorld => {
                self.request_to_create_new_world(ctx);
                true
            }
            Msg::Void(render) => render,
            Msg::EventPublished => {
                // TODO this should be optimized
                if let Some(world_id) = self.world_id.as_ref() {
                    self.request_to_get_world(ctx, *world_id);
                }
                true
            }
            Msg::NotificationRecieved(data) => {
                let narrator = narrator::Cake::default();
                if let Ok(json) = serde_json::from_str(&data) {
                    let json: Value = json;
                    if let Some(_event) = narrator.parse_event(json.clone()) {
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
                //
                // Note that notification needs to be send once
                // status_scope in initialized by the subcomponent
                let status_scope = self.status_scope.clone();

                self.ws_queue.drain(..).for_each(|msg| {
                    let status_scope = status_scope.clone();
                    ctx.link().send_future(async move {
                        log::warn!("{:?}", status_scope);
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
                <Print {close_cb} items={make_print_items(&Self::make_world())}/>
            };
        }

        if let Some(world) = &self.world {
            let available_characters = make_characters(&world);
            let set_character_callback = link
                .callback(|selected_character| Msg::UpdateSelectedCharacter(selected_character));
            let narrator = narrator::Cake::default();
            let events = narrator.available_events(world);
            let characters_map: HashMap<String, Rc<characters::Character>> =
                make_characters(&world)
                    .iter()
                    .map(|c| (c.name.to_string(), c.clone()))
                    .collect();

            let events: Vec<Rc<ActionEventItem>> = events
                .into_iter()
                .enumerate()
                .map(|(idx, e)| {
                    Rc::new(ActionEventItem::new(
                        idx,
                        e.action_text(world),
                        characters_map.get(&e.initiator()).unwrap().clone(),
                        None,
                        None,
                        serde_json::to_vec(&e.dump()).unwrap(),
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

            let fixed_character: Option<String> =
                storage::LocalStorage::get("fixed_character").ok();

            html! {
                <>
                    <section class="hero is-small is-light">
                      <div class="hero-body">
                          <p class="title">
                            {world.description().short(world)}
                          </p>
                          <div class="subtitle is-flex">
                              <div class="w-100 field is-grouped is-grouped-multiline is-justify-content-center">
                                  <div class="has-text-centered">
                                      <Status
                                        world_id={self.world_id.clone()}
                                        namespace={"some_namespace"}
                                        story={"doggie_and_kitie_cake"}
                                        msg_recieved={notification_cb}
                                        status_ready={status_ready_cb}
                                        status_scope={self.status_scope.clone()}
                                      />
                                  </div>
                                  <div class="has-text-centered">
                                      <Speech
                                        lang={lang.clone()}
                                        start_text={world.description().short(world)}
                                        shared_scope={self.speech_scope.clone()}
                                        world_name={world.name()}
                                      />
                                  </div>
                                  <div class="has-text-centered">
                                      <button class="button is-outlined" onclick={reset_cb}>
                                          <span class="icon has-text-danger">
                                              <i class="fas fa-sign-out-alt"></i>
                                          </span>
                                      </button>
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
                        <Messages shared_scope={ self.messages_scope.clone() }/>
                        { self.view_scene(ctx) }
                        <Actions
                          lang={ lang }
                          available_characters={ available_characters }
                          owned_items={ make_owned_items(world, self.selected_character.as_ref()) }
                          selected_character={ self.selected_character.clone() }
                          events={ events }
                          trigger_event={ trigger_event_callback }
                          trigger_scanned_event={ trigger_scanned_event_callback }
                          world_id={self.world_id.unwrap_or_default().clone()}
                          actions_scope={self.actions_scope.clone()}
                        />
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

            let world = Self::make_world();

            html! {
                <>
                    <Intro
                        new_world={new_world_cb}
                        story_name={world.description().short(&world)}
                        story_detail={world.description().long(&world)}
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
    fn screen_text(&self) -> Option<String> {
        let world = self.world.as_ref()?;
        if let Some(character) = self.selected_character.as_ref() {
            let character = world.characters().get(character).unwrap();
            let scene_name = character.scene().as_ref().unwrap();
            let scene = world.scenes().get(scene_name).unwrap();
            Some(scene.long(world))
        } else {
            None
        }
    }

    fn view_scene(&self, ctx: &Context<Self>) -> Html {
        if let Some(world) = &self.world {
            let onclick = ctx.link().callback(|_| Msg::Reset);
            let restart_text = translations::get_message("restart", world.lang(), None);
            let end_text = translations::get_message("end", world.lang(), None);
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
                        <h1 class="title">{ scene.short(world) }</h1>
                        <p class="subtitle">
                            <article class="message">
                                <div class="message-body">
                                    { scene.long(world) }
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

    fn make_world() -> CakeWorld {
        let mut world = CakeWorldBuilder::make_world().unwrap();
        world.setup();
        world.set_lang("cs");
        world
    }

    fn request_to_create_new_world(&mut self, ctx: &Context<Self>) {
        *self.loading.borrow_mut() = true;
        let loading = self.loading.clone();
        ctx.link().send_future(async move {
            let url = "/api/some_namespace/doggie_and_kitie_cake/";
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
        ctx.link().send_future(async move {
            let url = format!("/api/some_namespace/doggie_and_kitie_cake/{}/", world_id);
            let res = make_request(&url, "GET", None).await;
            *loading.borrow_mut() = false;
            match res {
                Ok((Some(json), status)) => {
                    let mut world = Self::make_world();
                    match status {
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
                    }
                }
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
        ctx.link().send_future(async move {
            let url = format!(
                "/api/some_namespace/doggie_and_kitie_cake/{}/event/",
                world_id
            );
            let res = make_request(&url, "POST", Some(event_json)).await;
            *loading.borrow_mut() = false;
            match res {
                Ok((resp, status)) => {
                    match status {
                        e if 200 <= e && e < 300 => {
                            log::debug!("Event triggered {:?}", resp);
                            Msg::EventPublished
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
