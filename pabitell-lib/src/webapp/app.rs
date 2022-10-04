use geo::Point;
use gloo::{
    storage::{self, Storage},
    timers::callback::Timeout,
};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::OrientationLockType;
use yew::prelude::*;

use crate::{
    events, protocol, translations,
    webapp::{
        action_event::ActionEventItem,
        actions::{Actions, Msg as ActionsMsg},
        character_switch::CharacterSwitch,
        characters, database,
        editor::Editor,
        geo_navigator::{make_navigations_data, GeoNavigator},
        intro::{FailedLoadState, Intro},
        items::Item,
        language_switch::LanguageSwitch,
        message::{Kind as MessageKind, MessageItem},
        messages::{Messages, Msg as MessagesMsg},
        print::{Print, PrintItem},
        scenes::make_scenes,
        speech::{Msg as SpeechMsg, Speech},
        status::{Status, WsStatus},
        websocket_client::{Msg as WsMsg, WebsocketClient},
    },
    Event, GeoLocation, Narrator, World,
};

pub type MakeCharacters = Option<Box<dyn Fn(&dyn World) -> Rc<Vec<Rc<characters::Character>>>>>;
pub type MakeNarrator = Option<Box<dyn Fn() -> Box<dyn Narrator>>>;
pub type MakePrintItems = Option<Box<dyn Fn(Box<dyn World>) -> Vec<PrintItem>>>;
pub type MakeOwnedItems = Option<Box<dyn Fn(&dyn World, &Option<String>) -> Vec<Rc<Item>>>>;
pub type MakeWorld = Option<Box<dyn Fn(&str) -> Box<dyn World>>>;
pub type MakeLanguages = Option<Box<dyn Fn() -> Rc<Vec<String>>>>;

const WS_TIMEOUT: u32 = 3000; // ws timeout in ms

pub enum Msg {
    UpdateCharacter(Rc<Option<String>>),
    TriggerEventData(Value),
    TriggerRestoreCharacter(Option<String>, bool, Uuid),
    PlayText(String),
    Leave,
    Reset,
    CreateNewWorld,
    WorldUpdateFetched(Box<dyn World>, bool),
    RefreshWorld,
    WsFlush,
    WsMessageRecieved(String),
    WsGetWorld(Uuid),
    WsTriggerEvent(Uuid, Value),
    WsNotifyUpdateWorld,
    WsFailed,
    WsConnect(Uuid),
    WsStatusUpdate(WsStatus),
    WsDisconnect,
    ShowPrint(bool),
    ScreenOrientationLocked(Option<JsValue>),
    SetLanguage(String),
    LoadWorld(Value),
    SetFailedLoadState(FailedLoadState),
    PositionReached(String, Point),
    UpdateSceneLocation(String, Option<Point>),
    ShowEditor(bool),
}

pub struct App {
    world_id: Option<Uuid>,
    world: Option<Box<dyn World>>,
    lang: String,
    owned: Option<bool>,
    /// Request which is being currently processed
    request_id: Option<(Uuid, Timeout)>,
    /// Current character tab
    character: Rc<Option<String>>,
    /// If it is sent can't switch to other characters
    fixed_character: bool,
    messages_scope: Rc<RefCell<Option<html::Scope<Messages>>>>,
    speech_scope: Rc<RefCell<Option<html::Scope<Speech>>>>,
    client_scope: Rc<RefCell<Option<html::Scope<WebsocketClient>>>>,
    actions_scope: Rc<RefCell<Option<html::Scope<Actions>>>>,
    ws_queue: Vec<String>,
    loading: Rc<RefCell<bool>>,
    show_print: bool,
    orientation_lock: Option<JsValue>,
    event_count: usize,
    ws_status: WsStatus,
    ws_request_failed: bool,
    load_failed: Option<FailedLoadState>,
    show_editor: bool,
}

#[derive(Properties, Default)]
pub struct Props {
    pub make_characters: MakeCharacters,
    pub make_narrator: MakeNarrator,
    pub make_print_items: MakePrintItems,
    pub make_owned_items: MakeOwnedItems,
    pub make_world: MakeWorld,
    pub make_languages: MakeLanguages,
    pub name: String,
}

impl PartialEq for Props {
    fn eq(&self, _other: &Self) -> bool {
        true
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

        // Make sure that WS connections are disconnected
        ctx.link().send_future(async { Msg::WsDisconnect });

        let lang: String = (storage::LocalStorage::get("pabitell_lang") as Result<String, _>)
            .unwrap_or_else(|_| "en".to_string());

        let mut res = Self {
            world_id,
            world: None,
            owned: None,
            request_id: None,
            character: Rc::new(storage::LocalStorage::get("character").ok()),
            fixed_character: storage::LocalStorage::get("fixed_character")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            messages_scope: Rc::new(RefCell::new(None)),
            speech_scope: Rc::new(RefCell::new(None)),
            client_scope: Rc::new(RefCell::new(None)),
            actions_scope: Rc::new(RefCell::new(None)),
            ws_queue: vec![],
            loading: Rc::new(RefCell::new(false)),
            show_print: false,
            orientation_lock: None,
            event_count: 0,
            ws_status: WsStatus::default(),
            ws_request_failed: false,
            lang,
            load_failed: None,
            show_editor: false,
        };

        if let Some(world_id) = world_id.as_ref() {
            log::info!("World Id: {:?}", &world_id);

            let world_id_owned = world_id.to_owned();
            // Start WS connection
            ctx.link()
                .send_future(async move { Msg::WsConnect(world_id_owned) });
            // Update data from the server
            res.request_to_get_world(ctx, *world_id);
        }

        res
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateCharacter(character) => {
                if let Some(character) = character.as_ref() {
                    storage::LocalStorage::set("character", character).unwrap();
                } else {
                    storage::LocalStorage::delete("character");
                }
                self.character = character;
                if let Some(world) = &self.world {
                    if let Some(character) = self.character.as_ref() {
                        let character = world.characters().get(character).unwrap();
                        if let Some(scene_name) = character.scene().as_ref() {
                            let scene = world.scenes().get(scene_name).unwrap();
                            ctx.link()
                                .send_message(Msg::PlayText(scene.long(world.as_ref())));
                        }
                    }
                    true
                } else {
                    false
                }
            }
            Msg::TriggerEventData(json_value) => {
                let value = json_value.clone();
                let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                if let Some(world) = self.world.as_mut() {
                    if let Some(mut event) = narrator.parse_event(world.as_ref(), json_value) {
                        // Update initiator
                        if let Some(character) = self.character.clone().as_ref() {
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

                        self.request_to_trigger_event(ctx, self.world_id.unwrap(), event.dump());
                    } else {
                        // Can't construct event based on given data
                        // TODO some error message
                        log::warn!("Failed to parse event from {}", value);
                        return false;
                    }
                } else {
                    return false;
                }
                false
            }
            Msg::TriggerRestoreCharacter(character, fixed_character, world_id) => {
                // check whether character exists in the world
                let world = ctx.props().make_world.as_ref().unwrap()(&self.lang);
                self.load_failed = None;

                // update charactes
                storage::LocalStorage::set(
                    "fixed_character",
                    if fixed_character { "true" } else { "false" },
                )
                .unwrap();
                self.fixed_character = fixed_character;

                if let Some(character) = character {
                    storage::LocalStorage::set("character", &character).unwrap();
                    self.character = Rc::new(Some(character.clone()));

                    if let Some(character_instance) = world.characters().get(&character) {
                        storage::LocalStorage::set("world_id", world_id).unwrap();
                        self.world_id = Some(world_id);

                        let world_id_cloned = world_id;
                        // Start ws connection
                        ctx.link()
                            .send_future(async move { Msg::WsConnect(world_id_cloned) });

                        // Get the world
                        self.request_to_get_world(ctx, world_id);

                        // Queue character notification
                        // we need to wait till Status component is initialized
                        self.ws_queue.push(
                            serde_json::to_string(&protocol::Message::Notification(
                                protocol::NotificationMessage::Joined(
                                    protocol::JoinedNotification {
                                        character: character_instance.dump(),
                                    },
                                ),
                            ))
                            .unwrap(),
                        );

                        true
                    } else {
                        log::warn!("Character '{}' is not found", character);
                        // TODO display message that character is not found
                        false
                    }
                } else {
                    self.world_id = Some(world_id);
                    storage::LocalStorage::set("world_id", world_id).unwrap();
                    storage::LocalStorage::delete("character");
                    self.character = Rc::new(None);

                    let world_id_cloned = world_id;
                    // Start ws connection
                    ctx.link()
                        .send_future(async move { Msg::WsConnect(world_id_cloned) });

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
            Msg::Leave => {
                self.world = None;
                storage::LocalStorage::delete("world_id");
                storage::LocalStorage::delete("fixed_character");
                storage::LocalStorage::delete("character");
                if let Some(scope) = self.messages_scope.as_ref().borrow().clone() {
                    scope.send_message(MessagesMsg::Clear);
                }

                // Disconnect from WS
                ctx.link().send_message(Msg::WsDisconnect);
                // clear picked character
                self.character = Rc::new(None);
                // clear owned flag
                self.owned = None;

                // clear loding state
                *self.loading.borrow_mut() = false;
                self.ws_request_failed = false;

                true
            }
            Msg::Reset => {
                // Only for owned world otherwise ignore
                if self.owned == Some(true) {
                    if let Some(orig_world) = self.world.as_ref() {
                        log::debug!("World {} is going to be reseted", &orig_world.id());
                        let mut world = ctx.props().make_world.as_ref().unwrap()(&self.lang);
                        world.load(orig_world.dump()).unwrap();
                        world.reset();

                        let name = ctx.props().name.clone();
                        let link = ctx.link().clone();

                        // Update db and send notifications
                        spawn_local(async move {
                            let db = database::init_database(&name).await;
                            database::put_world(
                                &db,
                                world.id(),
                                "narrator".to_string(),
                                false,
                                world.dump(),
                                true,
                                world.version(),
                            )
                            .await
                            .unwrap();
                            link.send_message(Msg::WorldUpdateFetched(world, true));
                            link.send_message(Msg::WsNotifyUpdateWorld);
                        });
                        return true;
                    }
                }
                false
            }
            Msg::WorldUpdateFetched(world, owned) => {
                self.owned = Some(owned);
                let old_screen_text = App::screen_text(&self.world, &self.character);
                self.event_count = world.event_count();
                self.world_id = Some(world.id().to_owned());
                self.world = Some(world);
                let new_screen_text = App::screen_text(&self.world, &self.character);
                if new_screen_text != old_screen_text {
                    if let Some(text) = new_screen_text {
                        ctx.link().send_message(Msg::PlayText(text));
                    }
                }

                // Clear loding state
                *self.loading.borrow_mut() = false;
                self.ws_request_failed = false;
                true
            }
            Msg::CreateNewWorld => {
                self.fixed_character = false;
                let mut world = ctx.props().make_world.as_ref().unwrap()(&self.lang);
                self.load_failed = None;
                world.setup(true);
                let name = ctx.props().name.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let db = database::init_database(&name).await;
                    database::put_world(
                        &db,
                        world.id(),
                        "narrator".to_string(),
                        false,
                        world.dump(),
                        true,
                        world.version(),
                    )
                    .await
                    .unwrap();
                    link.send_message(Msg::WsConnect(world.id().to_owned()));
                    link.send_message(Msg::WorldUpdateFetched(world, true))
                });
                true
            }
            Msg::RefreshWorld => {
                if let Some(world_id) = self.world_id {
                    self.request_to_get_world(ctx, world_id);
                }
                true
            }
            Msg::WsMessageRecieved(data) => {
                let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                match serde_json::from_str(&data) {
                    Ok(protocol::Message::Notification(notification)) => {
                        match notification {
                            protocol::NotificationMessage::Event(event_notification) => {
                                let world = if let Some(world) = self.world.as_ref() {
                                    world
                                } else {
                                    return false;
                                };
                                if let Some(event) =
                                    narrator.parse_event(world.as_ref(), event_notification.event)
                                {
                                    self.event_count = event_notification.event_count;
                                    log::info!("New event arrived from ws");

                                    // Store event to database
                                    let name = ctx.props().name.clone();
                                    let world_id = self.world_id.unwrap();
                                    let event_count = self.event_count;
                                    let data = event.dump();
                                    spawn_local(async move {
                                        let db = database::init_database(&name).await;
                                        database::put_event(
                                            &db,
                                            &world_id,
                                            event_count as u64,
                                            data,
                                        )
                                        .await
                                        .unwrap();
                                    });

                                    if let Some(actions_scope) =
                                        self.actions_scope.as_ref().borrow().as_ref()
                                    {
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
                                                translations::get_message_global(
                                                    "event",
                                                    world.lang(),
                                                    None,
                                                ),
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
                                                .send_message(MessagesMsg::AddMessage(Rc::new(
                                                    message,
                                                )));
                                            if !event.get_tags().contains(&"no_read".to_string()) {
                                                ctx.link()
                                                    .send_message(Msg::PlayText(text.to_string()));
                                            }
                                        }
                                    }
                                }
                            }
                            protocol::NotificationMessage::Joined(joined_notification) => {
                                log::info!(
                                    "Character '{}' joined",
                                    joined_notification.character["name"]
                                );
                                if let Some(actions_scope) =
                                    self.actions_scope.as_ref().borrow().as_ref()
                                {
                                    actions_scope.send_message(ActionsMsg::QRCodeHide);
                                }
                            }
                            protocol::NotificationMessage::WorldUpdate => {
                                if let Some(world) = self.world.as_ref() {
                                    if !self.owned.unwrap_or(false) {
                                        log::info!("World update performed. Fetch new world");
                                        ctx.link()
                                            .send_message(Msg::WsGetWorld(world.id().to_owned()));
                                    }
                                }
                            }
                        }
                    }
                    Ok(protocol::Message::Request(request)) => {
                        if self.owned == Some(true) {
                            let name = ctx.props().name.clone();
                            let client_scope = self.client_scope.clone();
                            match request {
                                protocol::RequestMessage::GetWorld(get_world) => {
                                    ctx.link().send_future(async move {
                                        let protocol::GetWorldRequest { msg_id, world_id } =
                                            get_world;

                                        let db = database::init_database(&name).await;

                                        let world = database::get_world(&db, &world_id)
                                            .await
                                            .unwrap()
                                            .map(|record| record["data"].clone());

                                        let resp = protocol::Message::Response(
                                            protocol::ResponseMessage::GetWorld(
                                                protocol::GetWorldResponse { msg_id, world },
                                            ),
                                        );
                                        if let Some(client_scope) =
                                            client_scope.as_ref().borrow().as_ref()
                                        {
                                            client_scope.send_message(WsMsg::SendMessage(
                                                serde_json::to_string(&resp).unwrap(),
                                            ));
                                        }

                                        Msg::WsFlush
                                    });
                                }
                                protocol::RequestMessage::TriggerEvent(trigger_event) => {
                                    let mut world =
                                        ctx.props().make_world.as_ref().unwrap()(&self.lang);
                                    let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                                    ctx.link().send_future(async move {
                                        let protocol::TriggerEventRequest {
                                            msg_id,
                                            world_id,
                                            event,
                                        } = trigger_event;
                                        let db = database::init_database(&name).await;

                                        let success = if let Some(record) =
                                            database::get_world(&db, &world_id.clone())
                                                .await
                                                .unwrap()
                                        {
                                            // First try to get world from database
                                            world.load(record["data"].clone()).unwrap();
                                            world.set_id(world_id);
                                            if let Some(mut event) =
                                                narrator.parse_event(world.as_ref(), event)
                                            {
                                                if event.can_be_triggered(world.as_ref()) {
                                                    // Apply event
                                                    // Send response
                                                    event.trigger(world.as_mut());

                                                    // Store world
                                                    database::put_world(
                                                        &db,
                                                        world.id(),
                                                        "narrator".to_string(),
                                                        false,
                                                        world.dump(),
                                                        true,
                                                        world.version(),
                                                    )
                                                    .await
                                                    .unwrap();

                                                    // Send notification that event was triggered
                                                    let notification =
                                                        protocol::Message::Notification(
                                                            protocol::NotificationMessage::Event(
                                                                protocol::EventNotification {
                                                                    event: event.dump(),
                                                                    event_count: world
                                                                        .event_count(),
                                                                },
                                                            ),
                                                        );
                                                    if let Some(client_scope) =
                                                        client_scope.as_ref().borrow().as_ref()
                                                    {
                                                        client_scope.send_message(
                                                            WsMsg::SendMessage(
                                                                serde_json::to_string(
                                                                    &notification,
                                                                )
                                                                .unwrap(),
                                                            ),
                                                        );
                                                    }

                                                    true
                                                } else {
                                                    false
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        };

                                        if let Some(client_scope) =
                                            client_scope.as_ref().borrow().as_ref()
                                        {
                                            let resp = protocol::Message::Response(
                                                protocol::ResponseMessage::TriggerEvent(
                                                    protocol::TriggerEventResponse {
                                                        msg_id,
                                                        success,
                                                    },
                                                ),
                                            );

                                            client_scope.send_message(WsMsg::SendMessage(
                                                serde_json::to_string(&resp).unwrap(),
                                            ));
                                        }

                                        Msg::WsFlush
                                    })
                                }
                            }
                        }
                    }
                    Ok(protocol::Message::Response(response)) => match response {
                        protocol::ResponseMessage::GetWorld(get_world) => {
                            let mut world = ctx.props().make_world.as_ref().unwrap()(&self.lang);
                            let world_id = self.world_id;

                            if let Some((msg_id, timeout)) = self.request_id.take() {
                                if get_world.msg_id == msg_id {
                                    timeout.cancel();
                                    *self.loading.borrow_mut() = false;

                                    let fixed_character = self.fixed_character;
                                    let character: Option<String> = self.character.as_ref().clone();
                                    let name = ctx.props().name.clone();

                                    if let Some(world_data) = get_world.world {
                                        ctx.link().send_future(async move {
                                            let world_id = world_id.unwrap();
                                            world.set_id(world_id);
                                            world.load(world_data).unwrap();
                                            log::debug!("World {} updated", world_id);
                                            let db = database::init_database(&name).await;
                                            database::put_world(
                                                &db,
                                                &world_id,
                                                character.unwrap_or_else(|| "narrator".to_string()),
                                                fixed_character,
                                                world.dump(),
                                                false,
                                                world.version(),
                                            )
                                            .await
                                            .unwrap();
                                            Msg::WorldUpdateFetched(world, false)
                                        });
                                    } else {
                                        // This branch should not be normally reached
                                        // usually if response comes it should contain data
                                        // aways reset is quite good response in this situation,
                                        // because world on onwer is lost...
                                        ctx.link().send_message(Msg::Leave);
                                    }
                                } else {
                                    self.request_id = Some((msg_id, timeout));
                                }
                            }
                        }
                        protocol::ResponseMessage::TriggerEvent(trigger_event) => {
                            if let Some((msg_id, timeout)) = self.request_id.take() {
                                if trigger_event.msg_id == msg_id {
                                    timeout.cancel();
                                    *self.loading.borrow_mut() = false;
                                    ctx.link().send_message(Msg::RefreshWorld);
                                } else {
                                    self.request_id = Some((msg_id, timeout));
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::warn!("Failed to parse WS data: {}", err);
                    }
                }
                false
            }
            Msg::WsFlush => {
                // Send notification to other connected clients
                // Note that notification needs to be send once
                // client_scope in initialized by the subcomponent
                let client_scope = self.client_scope.clone();

                self.ws_queue.drain(..).for_each(|msg| {
                    let client_scope = client_scope.clone();
                    spawn_local(async move {
                        if let Some(client_scope) = client_scope.as_ref().borrow().as_ref() {
                            client_scope.send_message(WsMsg::SendMessage(msg));
                        }
                    });
                });
                false
            }
            Msg::WsGetWorld(world_id) => {
                log::debug!("Obtaining world {} through WS.", world_id);

                *self.loading.borrow_mut() = true;
                let link = ctx.link().clone();
                let msg_id = Uuid::new_v4();

                // Set world_id to be sure that a proper world is fetched
                self.world_id = Some(world_id);

                self.request_id = Some((
                    msg_id,
                    Timeout::new(WS_TIMEOUT, move || link.send_message(Msg::WsFailed)),
                ));

                self.ws_queue.push(
                    serde_json::to_string(&protocol::Message::Request(
                        protocol::RequestMessage::GetWorld(protocol::GetWorldRequest {
                            msg_id,
                            world_id,
                        }),
                    ))
                    .unwrap(),
                );

                // Plan flushing of WS messages
                ctx.link().send_future(async { Msg::WsFlush });

                true
            }
            Msg::WsNotifyUpdateWorld => {
                // Just send notification that the reset of the world was performed
                self.ws_queue.push(
                    serde_json::to_string(&protocol::Message::Notification(
                        protocol::NotificationMessage::WorldUpdate,
                    ))
                    .unwrap(),
                );

                // Plan flushing of WS messages
                ctx.link().send_future(async { Msg::WsFlush });
                true
            }
            Msg::WsTriggerEvent(world_id, event) => {
                *self.loading.borrow_mut() = true;
                let msg_id = Uuid::new_v4();
                let link = ctx.link().clone();
                self.request_id = Some((
                    msg_id,
                    Timeout::new(WS_TIMEOUT, move || link.send_message(Msg::WsFailed)),
                ));

                self.ws_queue.push(
                    serde_json::to_string(&protocol::Message::Request(
                        protocol::RequestMessage::TriggerEvent(protocol::TriggerEventRequest {
                            msg_id,
                            world_id,
                            event,
                        }),
                    ))
                    .unwrap(),
                );

                // Plan flushing of WS messages
                ctx.link().send_future(async { Msg::WsFlush });

                true
            }
            Msg::WsFailed => {
                *self.loading.borrow_mut() = false;
                self.ws_request_failed = true;
                true
            }
            Msg::WsConnect(world_id) => {
                let client_scope = self.client_scope.clone();
                if let Some(client_scope) = client_scope.as_ref().borrow().as_ref() {
                    client_scope.send_message(WsMsg::Connect(world_id));
                }
                true
            }
            Msg::WsStatusUpdate(status) => {
                log::debug!("Ws Status update {:?}->{:?}", self.ws_status, &status);
                if WsStatus::CONNECTED == status && self.owned == Some(false) {
                    // Plan to redownload world
                    if let Some(world_id) = self.world_id {
                        ctx.link()
                            .send_future(async move { Msg::WsGetWorld(world_id) });
                    }
                }
                self.ws_status = status;
                true
            }
            Msg::WsDisconnect => {
                let client_scope = self.client_scope.clone();
                log::debug!("Disconnecting WS");
                if let Some(client_scope) = client_scope.as_ref().borrow().as_ref() {
                    client_scope.send_message(WsMsg::Disconnect);
                }
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
            Msg::SetLanguage(lang) => {
                log::debug!("Setting language to '{lang}'");
                let _ = storage::LocalStorage::set("pabitell_lang", &lang); // best effort
                self.lang = lang;
                if let Some(world) = self.world.as_mut() {
                    world.set_lang(&self.lang);
                }

                true
            }
            Msg::LoadWorld(world_json) => {
                self.fixed_character = false;
                self.load_failed = None;
                let mut world = ctx.props().make_world.as_ref().unwrap()(&self.lang);
                if world.load(world_json).is_ok() {
                    let name = ctx.props().name.clone();
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        let db = database::init_database(&name).await;
                        database::put_world(
                            &db,
                            world.id(),
                            "narrator".to_string(),
                            false,
                            world.dump(),
                            true,
                            world.version(),
                        )
                        .await
                        .unwrap();
                        link.send_message(Msg::WsConnect(world.id().to_owned()));
                        link.send_message(Msg::WorldUpdateFetched(world, true))
                    });
                } else {
                    ctx.link()
                        .send_message(Msg::SetFailedLoadState(FailedLoadState::WrongJson));
                }
                true
            }
            Msg::SetFailedLoadState(state) => {
                self.load_failed = Some(state);
                true
            }
            Msg::PositionReached(character, point) => {
                if let Some(world) = self.world.as_ref() {
                    log::debug!("Character {} reached {:?}", character, point);
                    // same character and destination
                    let narrator = ctx.props().make_narrator.as_ref().unwrap()();
                    let geo_location: GeoLocation = point.into();

                    // Trigger matched events
                    narrator
                        .available_events_sorted(world.as_ref())
                        .into_iter()
                        .filter(|e| {
                            if let Some((ch, _, p)) = e.geo_location(world.as_ref()) {
                                ch == character && p == geo_location
                            } else {
                                false
                            }
                        })
                        .for_each(|e| ctx.link().send_message(Msg::TriggerEventData(e.dump())));

                    true
                } else {
                    false
                }
            }
            Msg::UpdateSceneLocation(scene_name, point_opt) => {
                if let Some(world) = self.world.as_mut() {
                    // update scene
                    let scene = world.scenes_mut().get_mut(&scene_name).unwrap();
                    if let Some(point) = point_opt {
                        scene.set_geo_location(Some(point.into()));
                    } else {
                        scene.set_geo_location(None);
                    }
                    let world_data = world.dump();
                    let world_id = world.id().to_owned();
                    let world_version = world.version();
                    let name = ctx.props().name.clone();
                    let link = ctx.link().clone();

                    spawn_local(async move {
                        let db = database::init_database(&name).await;
                        let world_id = world_id; // move world_id inside this closure
                        database::put_world(
                            &db,
                            &world_id,
                            "narrator".to_string(),
                            false,
                            world_data,
                            true,
                            world_version,
                        )
                        .await
                        .unwrap();
                        // Notifi connected client that the location changed
                        link.send_message(Msg::WsNotifyUpdateWorld);
                    });

                    true
                } else {
                    false
                }
            }
            Msg::ShowEditor(show) => {
                if Some(true) == self.owned {
                    self.show_editor = show;
                    true
                } else {
                    log::warn!("Trying to show editor for not owned world");
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();

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
                    props.make_print_items.as_ref().unwrap()(
                        props.make_world.as_ref().unwrap()(&self.lang)
                    )
                  }
                />
            };
        }

        let set_language_cb = link.callback(Msg::SetLanguage);

        let view = if let Some(world) = &self.world {
            let available_characters = props.make_characters.as_ref().unwrap()(world.as_ref());
            let set_character_callback = link.callback(Msg::UpdateCharacter);
            let narrator = props.make_narrator.as_ref().unwrap()();
            let events = narrator.available_events_sorted(world.as_ref());

            // Prepare geo navitions
            let nav_data = make_navigations_data(&events, world.as_ref())
                .into_iter()
                .filter(|e| Some(&e.0) == self.character.as_ref().as_ref())
                .collect::<Vec<_>>();
            let nav_element_html = |(character, scene_name, scene_title, point): (
                String,
                Option<String>,
                Option<String>,
                Point,
            )| {
                let character_cloned = character.to_string();
                let reached_cb =
                    link.callback(move |_| Msg::PositionReached(character_cloned.clone(), point));
                html! {
                  <div class="subtitle is-flex">
                      <div class="w-100 field is-justify-content-center">
                          <div class="has-text-centered">
                            <GeoNavigator
                              destination={point}
                              reached={reached_cb}
                              {character}
                              {scene_name}
                              {scene_title}
                            />
                          </div>
                      </div>
                  </div>
                }
            };

            let characters_map: HashMap<String, Rc<characters::Character>> =
                props.make_characters.as_ref().unwrap()(world.as_ref())
                    .iter()
                    .map(|c| (c.name.to_string(), c.clone()))
                    .collect();

            let events: Vec<Rc<ActionEventItem>> = events
                .into_iter()
                .enumerate()
                .map(|(idx, e)| {
                    let (image, self_triggering, geo_location) =
                        if let Some(event) = e.as_any().downcast_ref::<events::Pick>() {
                            (
                                Some(format!("images/{}.svg", event.item())),
                                false,
                                event.geo_location(world.as_ref()).map(|e| e.2),
                            )
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Give>() {
                            (
                                Some(format!("images/{}.svg", event.item())),
                                false,
                                event.geo_location(world.as_ref()).map(|e| e.2),
                            )
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Move>() {
                            (
                                Some(format!("images/{}.svg", event.scene())),
                                false,
                                event.geo_location(world.as_ref()).map(|e| e.2),
                            )
                        } else if let Some(event) = e.as_any().downcast_ref::<events::UseItem>() {
                            (
                                Some(format!("images/{}.svg", event.item())),
                                false,
                                event.geo_location(world.as_ref()).map(|e| e.2),
                            )
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Void>() {
                            (
                                event.item().as_ref().map(|e| format!("images/{}.svg", e)),
                                false,
                                event.geo_location(world.as_ref()).map(|e| e.2),
                            )
                        } else if let Some(event) = e.as_any().downcast_ref::<events::Talk>() {
                            (
                                Some("images/comment.svg".to_owned()),
                                true,
                                event.geo_location(world.as_ref()).map(|e| e.2),
                            )
                        } else {
                            (None, false, None)
                        };
                    Rc::new(ActionEventItem::new(
                        idx,
                        e.action_text(world.as_ref()),
                        characters_map.get(&e.initiator()).unwrap().clone(),
                        None,
                        image,
                        serde_json::to_vec(&e.dump()).unwrap(),
                        self_triggering,
                        self.character.is_none(),
                        geo_location,
                    ))
                })
                .collect();

            let trigger_event_data_callback = link.callback(Msg::TriggerEventData);

            let leave_cb = link.callback(|_| Msg::Leave);
            let reset_cb = link.callback(|_| Msg::Reset);
            let edit_cb = link.callback(|_| Msg::ShowEditor(true));
            let refresh_world_cb = link.callback(|_| Msg::RefreshWorld);

            let finished = if let Some(world) = self.world.as_ref() {
                world.finished()
            } else {
                false
            };
            let built_on_text = format!(
                "{}: {}",
                translations::get_message_global("built_on", world.lang(), None,),
                build_time::build_time_utc!("%Y-%m-%d %H:%M:%S"),
            );

            let mut owned_items =
                props.make_owned_items.as_ref().unwrap()(world.as_ref(), self.character.as_ref());
            owned_items.sort();
            let owned_items = Rc::new(owned_items);

            let world_id = world.id().to_owned();

            html! {
                <>
                    <section class="hero is-small is-light">
                      <div class="hero-body">
                          <p class="title">
                            {world.description().short(world.as_ref())}
                          </p>
                          <div class="subtitle is-flex">
                              <div class="w-100 field is-grouped is-grouped-multiline is-justify-content-center">
                                  <div class="has-text-centered is-flex">
                                      <LanguageSwitch
                                          languages={props.make_languages.as_ref().unwrap()()}
                                          {set_language_cb}
                                          lang={self.lang.clone()}
                                      />
                                      <Speech
                                        lang={self.lang.clone()}
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
                                        refresh_world={refresh_world_cb}
                                        leave_world={leave_cb}
                                        reset_world={reset_cb}
                                        edit_world={edit_cb}
                                        can_reset={self.owned.unwrap_or(false)}
                                        can_edit={self.owned.unwrap_or(false)}
                                        connect_ws={link.callback(move |_| Msg::WsConnect(world_id))}
                                        event_count={self.event_count}
                                        status={self.ws_status.clone()}
                                        ws_request_failed={self.ws_request_failed}
                                      />
                                  </div>
                              </div>
                          </div>
                          { for nav_data.into_iter().map(nav_element_html) }
                      </div>
                    </section>
                    <main>
                        <CharacterSwitch
                          available_characters={ available_characters.clone() }
                          set_character={ set_character_callback }
                          character={ self.character.clone() }
                          fixed={self.fixed_character}
                        />
                        <Actions
                          lang={ self.lang.clone() }
                          available_characters={ available_characters }
                          { owned_items }
                          character={ self.character.clone() }
                          events={ events }
                          trigger_event_data={ trigger_event_data_callback }
                          world_id={self.world_id.unwrap_or_default()}
                          actions_scope={self.actions_scope.clone()}
                          { finished }
                        />
                        { self.view_scene(ctx) }
                        <Messages shared_scope={ self.messages_scope.clone() }/>
                    </main>
                    <footer class="footer">
                        <div class="content has-text-centered">
                            <a href="https://github.com/shenek/pabitell/"> { "Pabitell" }</a>
                            <br />
                            <small class="is-italic is-size-7">{ built_on_text }</small>
                        </div>
                    </footer>
                    { loading }
                    { self.view_editor(ctx, world.as_ref()) }
                </>
            }
        } else {
            let new_world_cb = link.callback(|_| Msg::CreateNewWorld);
            let character_scanned_cb = link.callback(|(character, fixed_character, world_id)| {
                Msg::TriggerRestoreCharacter(character, fixed_character, world_id)
            });
            let show_print_cb = link.callback(Msg::ShowPrint);

            let world = props.make_world.as_ref().unwrap()(&self.lang);
            let available_characters = props.make_characters.as_ref().unwrap()(world.as_ref());
            let load_failed_cb = link.callback(|state| Msg::SetFailedLoadState(state));
            let try_load_world_cb = link.callback(|json_world| Msg::LoadWorld(json_world));
            html! {
                <>
                    <Intro
                        new_world={new_world_cb}
                        name={props.name.to_owned()}
                        {available_characters}
                        languages={props.make_languages.as_ref().unwrap()()}
                        set_language={set_language_cb}
                        lang={self.lang.clone()}
                        story_name={world.description().short(world.as_ref())}
                        story_detail={world.description().long(world.as_ref())}
                        character_scanned={character_scanned_cb}
                        show_print={show_print_cb}
                        load_failed={self.load_failed.clone()}
                        {load_failed_cb}
                        {try_load_world_cb}
                        world_version={world.version()}
                    />
                    { loading }
                </>
            }
        };

        let ws_ready_cb = link.callback(|_| Msg::WsFlush);
        let ws_message_cb = link.callback(Msg::WsMessageRecieved);

        html! {
            <>
                <WebsocketClient
                    namespace={"some_namespace"}
                    story={props.name.clone()}
                    msg_recieved={ws_message_cb}
                    client_scope={self.client_scope.clone()}
                    ready={ws_ready_cb}
                    connecting={link.callback(|_| Msg::WsStatusUpdate(WsStatus::CONNECTING))}
                    connected={link.callback(|_| Msg::WsStatusUpdate(WsStatus::CONNECTED))}
                    disconnected={link.callback(|_| Msg::WsStatusUpdate(WsStatus::DISCONNECTED))}
                />
                { view }
            </>
        }
    }
}

impl App {
    fn screen_text(world: &Option<Box<dyn World>>, character: &Option<String>) -> Option<String> {
        let world = world.as_ref()?;
        if let Some(character) = character.as_ref() {
            let character = world.characters().get(character).unwrap();
            if let Some(scene_name) = character.scene().as_ref() {
                let scene = world.scenes().get(scene_name).unwrap();
                Some(scene.long(world.as_ref()))
            } else {
                None
            }
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

            let scene_description = if let Some(character) = self.character.as_ref() {
                let character = world.characters().get(character).unwrap();
                if let Some(scene_name) = character.scene().as_ref() {
                    let scene = world.scenes().get(scene_name).unwrap();
                    let audio = if let Some(filename) = scene.music() {
                        html! {
                            <audio loop=true autoplay=true>
                                <source src={filename} type="audio/ogg"/>
                            </audio>
                        }
                    } else {
                        html! {}
                    };
                    html! {
                        <>
                            <h1 class="title">{ scene.short(world.as_ref()) }</h1>
                            { audio }
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
                }
            } else {
                html! {}
            };

            let class = if world.finished() || self.character.is_some() {
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

    fn view_editor(&self, ctx: &Context<Self>, world: &dyn World) -> Html {
        let close = ctx.link().callback(|_| Msg::ShowEditor(false));
        let update_location = ctx
            .link()
            .callback(|(scene, point_opt)| Msg::UpdateSceneLocation(scene, point_opt));
        let scenes = make_scenes(world);
        let title = translations::get_message_global("world_editor", world.lang(), None);
        let places_text = translations::get_message_global("places", world.lang(), None);
        if self.show_editor {
            html! {
                <Editor {close} {update_location} {scenes} {title} {places_text} />
            }
        } else {
            html! {}
        }
    }

    fn request_to_get_world(&mut self, ctx: &Context<Self>, world_id: Uuid) {
        *self.loading.borrow_mut() = true;
        let name = ctx.props().name.clone();
        let mut world = ctx.props().make_world.as_ref().unwrap()(&self.lang);
        world.set_id(world_id);
        let owned = self.owned;

        ctx.link().send_future(async move {
            // first try to detect whether the world is owned
            match owned {
                Some(true) => {
                    // World is owned obtain it from db
                    let db = database::init_database(&name).await;
                    if let Some(record) = database::get_world(&db, &world_id).await.unwrap() {
                        // First try to get world from database
                        world.load(record["data"].clone()).unwrap();

                        // If this world is owned update it right away
                        log::debug!("World {} is owned. Local db queried.", world_id);
                        Msg::WorldUpdateFetched(world, true)
                    } else {
                        // Got to intro page
                        Msg::Leave
                    }
                }
                Some(false) => {
                    // World is not owned, obtain it via WS
                    Msg::WsGetWorld(world_id)
                }
                None => {
                    let db = database::init_database(&name).await;
                    if let Some(record) = database::get_world(&db, &world_id).await.unwrap() {
                        match record["owned"].as_bool() {
                            Some(true) => {
                                // First try to get world from database
                                world.load(record["data"].clone()).unwrap();

                                // If this world is owned update it right away
                                log::debug!("World {} is owned. Local db queried.", world_id);
                                Msg::WorldUpdateFetched(world, true)
                            }
                            _ => {
                                // world is not owned get it using WS
                                Msg::WsGetWorld(world_id)
                            }
                        }
                    } else {
                        // World is not present it db, this means that it is not owned
                        // and we should try to get world from WS
                        Msg::WsGetWorld(world_id)
                    }
                }
            }
        });
    }

    fn request_to_trigger_event(&mut self, ctx: &Context<Self>, world_id: Uuid, event_json: Value) {
        *self.loading.borrow_mut() = true;
        // TODO owned world can be queried locally
        ctx.link().send_future(async move {
            log::debug!("Obtaining world {} through WS.", world_id);
            // World can't be owned => try to get it via websockets
            Msg::WsTriggerEvent(world_id, event_json)
        });
    }
}
