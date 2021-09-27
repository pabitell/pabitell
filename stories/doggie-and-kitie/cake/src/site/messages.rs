use data_url::{mime, DataUrl};
use pabitell_lib::{Character, Description, World, WorldBuilder};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

use super::{
    characters, message,
    qrcode::{Msg as QRCodeMsg, QRCode},
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

#[derive(Clone, Debug, Default, Properties)]
pub struct Props {
    pub shared_scope: Rc<RefCell<Option<html::Scope<Messages>>>>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, rhs: &Self) -> bool {
        true
    }
}

pub enum Msg {
    Close(usize),
    AddMessage(Rc<message::MessageItem>),
}

pub struct Messages {
    pub idx: usize,
    pub messages: HashMap<usize, Rc<message::MessageItem>>,
}

impl Component for Messages {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            messages: HashMap::new(),
            idx: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close(idx) => self.messages.remove(&idx).is_some(),
            Msg::AddMessage(message) => {
                self.idx += 1;
                self.messages.insert(self.idx, message);
                true
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
