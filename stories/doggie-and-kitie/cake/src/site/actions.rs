use pabitell_lib::{Character, Description, World, WorldBuilder};
use serde_json::Value;
use std::sync::Arc;
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

use super::characters;

#[derive(Debug, Clone, PartialEq)]
pub struct EventItem {
    idx: usize,
    description: String,
    character: characters::Character,
    action_url: Option<String>,
    image_url: Option<String>,
}

impl EventItem {
    pub fn new(
        idx: usize,
        description: String,
        character: characters::Character,
        action_url: Option<String>,
        image_url: Option<String>,
    ) -> Self {
        Self {
            idx,
            description,
            character,
            action_url,
            image_url,
        }
    }
}

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
        let scan_cb = link.callback(move |_| Msg::QRCodeScan);
        let render_action = move |item: EventItem| {
            let EventItem {
                idx,
                description,
                character,
                action_url,
                image_url,
            } = item;
            let cb = link.callback(move |_| Msg::TriggerEvent(idx));
            html! {
                <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                    <div class="card-content">
                        <div class="media">
                            <div class="media-left">
                                <figure class="image is-48x48">
                                    <img src={ character.character_url } alt="Character"/>
                                </figure>
                            </div>
                            <div class="media-content">
                                <p class="title is-4">{ character.short }</p>
                                <p class="subtitle is-6">{"TODO QR code ETC"}</p>
                            </div>
                        </div>
                        <div class="content">{description}</div>
                    </div>
                    <div class="card-image has-text-centered">
                        <figure onclick={ cb } class="image w-75 is-square is-clickable is-inline-block">
                            <img class="box" src={ image_url.unwrap_or_else(|| "svgs/solid/cog.svg".to_string()) } alt="Action image"/>
                        </figure>
                    </div>
                </div>
            }
        };
        html! {
            <section class="section is-flex">
                <div class="columns is-flex-wrap-wrap w-100">
                    <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                        <div class="card-content">
                            <div class="media">
                                <div class="media-left">
                                    <figure class="image is-48x48">
                                        <img src="svgs/solid/dog.svg" alt="Doggie"/>
                                    </figure>
                                </div>
                                <div class="media-content">
                                    <p class="title is-4">{"Doggie"}</p>
                                    <p class="subtitle is-6">{"TODO QR code ETC"}</p>
                                </div>
                            </div>
                            <div class="content">{"Scan QR Code"}</div>
                        </div>
                        <div class="card-image has-text-centered">
                            <figure onclick={ scan_cb } class="image is-clickable is-square w-75 is-inline-block box">
                                <img class="box" src="https://publicdomainvectors.org/download.php?file=Share-the-Openclipart-QR-Code.svg" alt="QR code"/>
                            </figure>
                        </div>
                    </div>
                    { for ctx.props().events.clone().into_iter().map(render_action) }
                </div>
            </section>
        }
    }
}
