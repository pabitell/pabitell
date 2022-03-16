// TODO it might be a good idea to use WS via webworker interface
// However agent are not mcuch stable in YEW yet so perhaps this should
// wait till yew 0.20.0 is released

use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message, WebSocketError};
use gloo::timers::callback::Timeout;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{html, prelude::*};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub world_id: Option<Uuid>,
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
        self.world_id == rhs.world_id
            && self.namespace == rhs.namespace
            && self.story == rhs.story
            && self.msg_recieved == rhs.msg_recieved
            && self.ready == rhs.ready
            && self.connected == rhs.connected
            && self.connecting == rhs.connecting
            && self.disconnected == rhs.disconnected
    }
}

pub struct WebsocketClient {
    sender: Option<SplitSink<WebSocket, Message>>,
    queued_messages: Vec<String>,
    reconnect_timeout: Option<Timeout>,
}

pub enum Msg {
    Connect,
    SetSender(SplitSink<WebSocket, Message>, bool),
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
            Msg::Connect => {
                log::debug!("Connecting to {:?}", &props.world_id);
                if let Some(url) = Self::ws_url(ctx) {
                    props.connecting.emit(());
                    let props = props.clone();
                    link.clone().send_future(async move {
                        match WebSocket::open(&url) {
                            Ok(ws) => {
                                log::debug!("WS opened {}", url);
                                let (sender, mut receiver) = ws.split();
                                link.send_message(Msg::SetSender(sender, true));
                                while let Some(msg) = receiver.next().await {
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
                                            break;
                                        }
                                        Err(WebSocketError::ConnectionError) => {
                                            log::warn!("WS connection Error {}", url);
                                            break;
                                        }
                                        Err(_) => {
                                            unreachable!()
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                log::warn!("WS error {}:{:?}", url, err);
                            }
                        }
                        Msg::Disconnect
                    });
                    true
                } else {
                    self.plan_reconnect(ctx);
                    false
                }
            }
            Msg::SetSender(sender, emit) => {
                self.sender = Some(sender);
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
            Msg::Disconnect => {
                log::warn!("Disconnect");
                let render = self.sender.is_some();
                props.disconnected.emit(());
                self.plan_reconnect(ctx);
                self.sender = None;
                render
            }
            Msg::SendMessage(message) => {
                log::debug!("Sending a message to websockets");
                if let Some(mut sender) = self.sender.take() {
                    ctx.link().send_future(async move {
                        // Best effort - don't care about the Result
                        let _ = sender.send(Message::Text(message)).await;

                        // Restore sender so other messages can be sent
                        Msg::SetSender(sender, false)
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
        // Plan to create a connection
        ctx.link().send_future(async { Msg::Connect });
        Self {
            sender: None,
            queued_messages: vec![],
            reconnect_timeout: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().client_scope.borrow_mut() = Some(ctx.link().clone());
        ctx.props().ready.emit(());
        true
    }
}

impl WebsocketClient {
    fn plan_reconnect(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().to_owned();
        self.reconnect_timeout = Some(Timeout::new(5000, move || {
            link.send_message(Msg::Connect);
        }));
    }

    fn ws_url(ctx: &Context<Self>) -> Option<String> {
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
            props.world_id?,
        ))
    }
}
