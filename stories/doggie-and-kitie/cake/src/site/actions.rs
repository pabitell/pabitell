use pabitell_lib::{Character, Description, World, WorldBuilder};
use serde_json::Value;
use std::sync::Arc;
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

pub type EventItem = (usize, String);

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {
    pub events: Vec<EventItem>,
    pub trigger_event: Callback<usize>,
}

pub enum Msg {
    QRCodeScan,
    TriggerEvent(usize),
}

pub struct Actions {}

impl Component for Actions {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TriggerEvent(idx) => {
                ctx.props().trigger_event.emit(idx);
            }
            QRCodeScan => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let render_action = move |(idx, item): (usize, String)| {
            let cb = link.callback(move |_| Msg::TriggerEvent(idx));
            html! {
                <div class="tile is-4">
                    <div class="card m-2">
                        <div class="card-image">
                            <figure class="image">
                                <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image"/>
                            </figure>
                        </div>
                        <div class="card-content">
                            <div class="media">
                                <div class="media-left">
                                    <figure class="image is-48x48">
                                        <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                                    </figure>
                                </div>
                                <div class="media-content">
                                    <p class="title is-4">{"Doggie"}</p>
                                    <p class="subtitle is-6">{"TODO QR code ETC"}</p>
                                </div>
                            </div>
                            <div class="content">{item}</div>
                            <div class="content">
                                <button onclick={ cb } class="button is-success is-fullwidth">{"Perform"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        };
        html! {
            <section class="section is-flex">
                <div class="tile is-ancestor is-flex-wrap-wrap">
                    <div class="tile is-4">
                        <div class="card m-2">
                            <div class="card-image">
                                <figure class="image">
                                    <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image"/>
                                </figure>
                            </div>
                            <div class="card-content">
                                <div class="media">
                                    <div class="media-left">
                                        <figure class="image is-48x48">
                                            <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                                        </figure>
                                    </div>
                                    <div class="media-content">
                                        <p class="title is-4">{"Doggie"}</p>
                                        <p class="subtitle is-6">{"TODO QR code ETC"}</p>
                                    </div>
                                </div>
                                <div class="content">{"Scan QR Code"}</div>
                                <div class="content">
                                    <a href="#" class="button is-success is-fullwidth">{"Scan"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                    { for ctx.props().events.clone().into_iter().map(render_action) }
                </div>
            </section>
        }
    }
}
