use std::{cell::RefCell, rc::Rc};
use yew::{html, prelude::*};

use super::{characters, qrcode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Ordinary,
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl AsRef<str> for Kind {
    fn as_ref(&self) -> &str {
        match self {
            Kind::Ordinary => "",
            Kind::Dark => "is-dark",
            Kind::Primary => "is-primary",
            Kind::Link => "is-link",
            Kind::Info => "is-info",
            Kind::Success => "is-success",
            Kind::Warning => "is-warning",
            Kind::Danger => "is-danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageItem {
    pub title: Rc<String>,
    pub text: Rc<String>,
    pub kind: Kind,
    pub icon: Rc<Option<String>>,
}

impl MessageItem {
    pub fn new(title: String, text: String, kind: Kind, icon: Option<String>) -> Self {
        Self {
            title: Rc::new(title),
            text: Rc::new(text),
            kind,
            icon: Rc::new(icon),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub item: Rc<MessageItem>,
    pub close_callaback: Callback<()>,
}

pub struct Message {}

pub enum Msg {
    Close,
}

impl Component for Message {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                ctx.props().close_callaback.emit(());
            }
        }
        false
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let close_cb = link.callback(|_| Msg::Close);

        let item = ctx.props().item.clone();
        let root_classes = classes!("message", item.kind.as_ref().to_string());

        let title = if let Some(icon) = item.icon.as_ref() {
            html! {
                <span class="icon-text">
                  <span class="icon">
                    <i class={ icon }></i>
                  </span>
                  <span>{ item.title.to_string() }</span>
                </span>
            }
        } else {
            html! {
                <p>{ item.title.to_string() }</p>
            }
        };
        html! {
            <article class={ root_classes }>
                <div class="message-header">
                { title }
                <button class="delete" aria-label="delete" onclick={ close_cb }></button>
                </div>
                <div class="message-body">{ item.text.to_string() }</div>
            </article>
        }
    }
}
