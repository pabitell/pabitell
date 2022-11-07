use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Notification, NotificationOptions, NotificationPermission};
use yew::prelude::*;

use super::message;

#[derive(Clone, Debug, Default, Properties)]
pub struct Props {
    pub shared_scope: Rc<RefCell<Option<html::Scope<Messages>>>>,
    pub lang: Rc<String>,
    pub world_name: Rc<String>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, _rhs: &Self) -> bool {
        true
    }
}

pub enum Msg {
    Close(usize),
    AddMessage(Rc<message::MessageItem>),
    NotificationsGranted,
    Clear,
}

pub struct Messages {
    pub idx: usize,
    pub messages: HashMap<usize, Rc<message::MessageItem>>,
    pub notifications_allowed: bool,
    pub permission_callback: Option<Closure<dyn Fn(NotificationPermission)>>,
}

impl Component for Messages {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        let (notifications_allowed, permission_callback) =
            if NotificationPermission::Granted != Notification::permission() {
                let link = ctx.link().clone();
                let cb = Closure::wrap(Box::new(move |permission: NotificationPermission| {
                    log::info!("Notification permissions updated");
                    if permission == NotificationPermission::Granted {
                        link.send_message(Msg::NotificationsGranted);
                    }
                }) as Box<dyn Fn(NotificationPermission)>);
                // Ask to grant permissions
                let _ = Notification::request_permission_with_permission_callback(
                    cb.as_ref().unchecked_ref(),
                );
                (false, Some(cb))
            } else {
                (true, None)
            };
        Self {
            messages: HashMap::new(),
            idx: 0,
            permission_callback,
            notifications_allowed,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close(idx) => self.messages.remove(&idx).is_some(),
            Msg::AddMessage(message) => {
                self.idx += 1;
                // Trigger notification
                if self.notifications_allowed {
                    let mut options = NotificationOptions::new();
                    options.body(&message.text);
                    options.icon("images/pabitell.svg");
                    options.lang(&ctx.props().lang);
                    options.tag(&ctx.props().world_name);
                    let _ = Notification::new_with_options(&message.title, &options);
                }
                self.messages.insert(self.idx, message);
                true
            }
            Msg::Clear => {
                self.messages.clear();
                true
            }
            Msg::NotificationsGranted => {
                self.notifications_allowed = true;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let messages = self.messages.clone();

        let mut keys = messages.keys().copied().collect::<Vec<usize>>();
        keys.sort();

        let render_message = move |(idx, item): (usize, &Rc<message::MessageItem>)| {
            let cb = link.callback(move |_| Msg::Close(idx));

            html! {
                <message::Message close_callaback={ cb } { item } />
            }
        };
        let classes = if self.messages.is_empty() {
            classes!("section", "is-hidden")
        } else {
            classes!("section")
        };
        html! {
            <section class={ classes }>
            { for keys.into_iter().map(|k| render_message((k, messages.get(&k).unwrap()))) }
            </section>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        true
    }
}
