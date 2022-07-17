// TODO it might be a good idea to use WS via webworker interface
// However agent are not mcuch stable in YEW yet so perhaps this should
// wait till yew 0.20.0 is released

use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message, WebSocketError};
use gloo::timers::callback::Timeout;
use std::{cell::RefCell, rc::Rc};
use stream_cancel::{StreamExt as CancelStreamExt, Trigger, Tripwire};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{html, prelude::*};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub namespace: String,
    pub story: String,
    pub msg_recieved: Callback<String>,
    pub ready: Callback<()>,
    pub connecting: Callback<()>,
    pub connected: Callback<()>,
    pub disconnected: Callback<()>,
    pub client_scope: Rc<RefCell<Option<html::Scope<WebsocketClient>>>>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, rhs: &Self) -> bool {
        self.namespace == rhs.namespace
            && self.story == rhs.story
            && self.msg_recieved == rhs.msg_recieved
            && self.ready == rhs.ready
            && self.connected == rhs.connected
            && self.connecting == rhs.connecting
            && self.disconnected == rhs.disconnected
    }
}

pub struct WebsocketClient {
    world_id: Option<Uuid>,
    sender: Option<(SplitSink<WebSocket, Message>, Trigger)>,
    queued_messages: Vec<String>,
    reconnect_timeout: Option<Timeout>,
}

pub enum Msg {
    Connect(Uuid),
    SetSender(SplitSink<WebSocket, Message>, Trigger, bool),
    /// Disconnected by e.g. failed connection
    Disconnected,
    /// Request to disconnect by the user
    Disconnect,
    SendMessage(String),
}

impl Component for WebsocketClient {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props();
        match msg {
            Msg::Connect(world_id) => {
                // Disconnect first
                if self.world_id.is_some() {
                    link.send_message(Msg::Disconnect);
                    link.send_message(Msg::Connect(world_id));
                    return true;
                }
                self.world_id = Some(world_id);
                log::debug!("Connecting to {:?}", &world_id);
                if let Some(url) = self.ws_url(ctx) {
                    props.connecting.emit(());
                    let props = props.clone();
                    spawn_local(async move {
                        let mut error = false;
                        let _ = &props;
                        match WebSocket::open(&url) {
                            Ok(ws) => {
                                log::debug!("WS opened {}", url);
                                // Terminate channel
                                let (trigger, tripwire) = Tripwire::new();

                                let (sender, receiver) = ws.split();
                                link.send_message(Msg::SetSender(sender, trigger, true));
                                let mut reader = receiver.take_until_if(tripwire);
                                while let Some(msg) = reader.next().await {
                                    log::debug!("MSG recieved: {:?}", &msg);
                                    match msg {
                                        Ok(Message::Text(msg)) => {
                                            props.msg_recieved.emit(msg);
                                        }
                                        Ok(Message::Bytes(msg)) => {
                                            log::warn!(
                                                "Binary data recieved from WS {} (len={})",
                                                url,
                                                msg.len()
                                            );
                                        }
                                        Err(WebSocketError::ConnectionClose(err)) => {
                                            log::warn!("WS closed {}:{:?}", url, err);
                                            error = true;
                                            break;
                                        }
                                        Err(WebSocketError::ConnectionError) => {
                                            log::warn!("WS connection Error {}", url);
                                            error = true;
                                            break;
                                        }
                                        Err(_) => {
                                            unreachable!()
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                error = true;
                                log::warn!("WS error {}:{:?}", url, err);
                            }
                        }
                        if error {
                            link.send_future(async { Msg::Disconnected });
                        }
                    });
                    true
                } else {
                    self.plan_reconnect(ctx, world_id);
                    false
                }
            }
            Msg::SetSender(sender, terminate, emit) => {
                self.sender = Some((sender, terminate));
                if emit {
                    log::info!("Ws connected");
                    props.connected.emit(());
                } else {
                    log::debug!("Restoring sender");
                }
                self.queued_messages.drain(..).for_each(|msg| {
                    log::debug!("Sending stored message {}", msg);
                    link.send_message(Msg::SendMessage(msg));
                });
                if let Some(timer) = self.reconnect_timeout.take() {
                    timer.cancel();
                }
                true
            }
            Msg::Disconnected => {
                log::debug!("WS disconneted. Reconnect was planned.");
                props.disconnected.emit(());
                if let Some(world_id) = self.world_id {
                    self.plan_reconnect(ctx, world_id);
                }
                true
            }
            Msg::Disconnect => {
                log::debug!("WS disconneted. Reconnect is not planned.");

                // Discard reconnecting
                if let Some(timer) = self.reconnect_timeout.take() {
                    timer.cancel();
                }

                // Drop sender to disconnect
                let resp = self.sender.take().is_some();

                // Unset world
                if self.world_id.is_some() {
                    self.world_id = None;
                    props.disconnected.emit(());
                }

                resp
            }
            Msg::SendMessage(message) => {
                log::debug!("Sending a message to websockets");
                if let Some((mut sender, trigger)) = self.sender.take() {
                    link.send_future(async move {
                        // Best effort - don't care about the Result
                        let _ = sender.send(Message::Text(message)).await;

                        // Restore sender so other messages can be sent
                        Msg::SetSender(sender, trigger, false)
                    });
                } else {
                    log::debug!("Add '{}' to queue", message);
                    self.queued_messages.push(message);
                }
                false
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().client_scope.borrow_mut() = Some(ctx.link().clone());
        ctx.props().ready.emit(());
        Self {
            sender: None,
            queued_messages: vec![],
            reconnect_timeout: None,
            world_id: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        ctx.props().ready.emit(());
        true
    }
}

impl WebsocketClient {
    fn plan_reconnect(&mut self, ctx: &Context<Self>, world_id: Uuid) {
        let link = ctx.link().to_owned();
        self.reconnect_timeout = Some(Timeout::new(5000, move || {
            link.send_message(Msg::Connect(world_id));
        }));
    }

    fn ws_url(&self, ctx: &Context<Self>) -> Option<String> {
        let location = web_sys::window().unwrap().location();
        let proto = if location.protocol().unwrap() == "https:" {
            "wss"
        } else {
            "ws"
        };
        let props = ctx.props();
        Some(format!(
            "{}://{}/ws/{}/{}/{}/",
            proto,
            location.host().unwrap(),
            props.namespace,
            props.story,
            self.world_id?,
        ))
    }
}
