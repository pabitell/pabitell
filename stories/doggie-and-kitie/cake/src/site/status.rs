use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{html, prelude::*, web_sys::Element};

use super::{characters, qrcode};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub world_id: Option<Uuid>,
    pub namespace: String,
    pub story: String,
    pub msg_recieved: Callback<String>,
    pub status_ready: Callback<()>,
    pub status_scope: Rc<RefCell<Option<html::Scope<Status>>>>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, rhs: &Self) -> bool {
        self.world_id == rhs.world_id
            && self.namespace == rhs.namespace
            && self.story == rhs.story
            && self.msg_recieved == rhs.msg_recieved
    }
}

#[derive(Clone, Debug)]
pub enum WsStatus {
    CONNECTING,
    CONNECTED,
    DISCONNECTED,
}

impl Default for WsStatus {
    fn default() -> Self {
        Self::DISCONNECTED
    }
}

impl WsStatus {
    fn icon_classes(&self) -> String {
        match self {
            Self::CONNECTED => "fas fa-check-circle",
            Self::CONNECTING => "rotate fas fa-circle-notch",
            Self::DISCONNECTED => "fas fa-times-circle",
        }
        .to_string()
    }
    fn text_classes(&self) -> String {
        match self {
            Self::CONNECTED => "icon has-text-success",
            Self::CONNECTING => "icon has-text-info",
            Self::DISCONNECTED => "icon has-text-danger",
        }
        .to_string()
    }
}

pub struct Status {
    status: WsStatus,
    ws: Option<WebSocket>,
    queued_messages: Vec<String>,
}

pub enum Msg {
    Connect,
    Connected(WebSocket),
    Disconnect,
    SendMessage(String),
}

impl Component for Status {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::Connect => {
                if let Some(url) = Self::ws_url(ctx) {
                    self.status = WsStatus::CONNECTING;
                    let props = ctx.props().clone();
                    link.clone().send_future(async move {
                        match WebSocket::open(&url) {
                            Ok(mut ws) => {
                                log::debug!("WS opened {}", url);
                                link.send_message(Msg::Connected(ws.clone()));
                                while let Some(msg) = ws.next().await {
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
                                        Err(err) => {
                                            log::warn!("WS error {}:{:?}", url, err);
                                            break;
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
                    false
                }
            }
            Msg::Connected(ws) => {
                log::info!("Ws connected");
                self.ws = Some(ws);
                self.status = WsStatus::CONNECTED;
                self.queued_messages.drain(..).for_each(|msg| {
                    log::debug!("Sending stored message {}", msg);
                    link.send_message(Msg::SendMessage(msg));
                });
                true
            }
            Msg::Disconnect => {
                log::warn!("Disconnect");
                let render = self.ws.is_some();
                self.status = WsStatus::DISCONNECTED;
                self.ws = None;
                render
            }
            Msg::SendMessage(message) => {
                log::debug!("Sending a message to websockets");
                if let Some(ws) = self.ws.as_ref() {
                    let mut ws = ws.clone();
                    spawn_local(async move {
                        // Best effort - don't care about the Result
                        let _ = ws.send(Message::Text(message)).await;
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
        *ctx.props().status_scope.borrow_mut() = Some(ctx.link().clone());
        ctx.props().status_ready.emit(());
        // Plan to create a connection
        ctx.link().send_future(async { Msg::Connect });
        Self {
            status: WsStatus::default(),
            ws: None,
            queued_messages: vec![],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Connect);

        html! {
            <button class="button is-outlined" {onclick}>
                <span class={ classes!(self.status.text_classes()) }>
                    <i class={ classes!(self.status.icon_classes()) }></i>
                </span>
            </button>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().status_scope.borrow_mut() = Some(ctx.link().clone());
        ctx.props().status_ready.emit(());
        true
    }
}

impl Status {
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
