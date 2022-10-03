use std::rc::Rc;
use yew::{html, prelude::*};

use crate::GeoLocation;

use super::characters;

#[derive(Debug, Clone, PartialEq)]
pub struct ActionEventItem {
    pub idx: usize,
    pub description: Rc<String>,
    pub character: Rc<characters::Character>,
    pub action_url: Rc<Option<String>>,
    pub image_url: Rc<Option<String>>,
    pub data: Rc<Vec<u8>>,
    pub self_triggering: bool,
    pub show_qr: bool,
    pub geo_location: Option<GeoLocation>,
}

impl ActionEventItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        idx: usize,
        description: String,
        character: Rc<characters::Character>,
        action_url: Option<String>,
        image_url: Option<String>,
        data: Vec<u8>,
        self_triggering: bool,
        show_qr: bool,
        geo_location: Option<GeoLocation>,
    ) -> Self {
        Self {
            idx,
            description: Rc::new(description),
            character,
            action_url: Rc::new(action_url),
            image_url: Rc::new(image_url),
            data: Rc::new(data),
            self_triggering,
            show_qr,
            geo_location,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub item: Rc<ActionEventItem>,
    pub trigger_event_cb: Callback<()>,
    pub show_qr_cb: Callback<Rc<Vec<u8>>>,
}

pub struct ActionEvent {}

pub enum Msg {
    ShowQRCode,
    TriggerEvent,
}

impl Component for ActionEvent {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowQRCode => {
                let data = ctx.props().item.data.clone();
                ctx.props().show_qr_cb.emit(data);
                false
            }
            Msg::TriggerEvent => {
                ctx.props().trigger_event_cb.emit(());
                true
            }
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = ctx.props().item.clone();
        let description = item.description.clone();
        let character = item.character.clone();
        let image_url = if let Some(url) = item.image_url.as_ref() {
            url.to_string()
        } else {
            "images/cog.svg".to_string()
        };

        let qr_button = if item.show_qr {
            let show_qr_cb = ctx.link().callback(|_| Msg::ShowQRCode);
            html! {
                <button class="button" onclick={ show_qr_cb } >
                    <i class="fas fa-qrcode"></i>
                </button>
            }
        } else {
            html! {}
        };

        let trigger_event_cb = ctx.link().callback(|_| Msg::TriggerEvent);
        html! {
            <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-48x48">
                                <img src={ character.character_url.to_string() } alt="Character"/>
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-4">{ character.short.to_string() }</p>
                            <p class="subtitle is-6">
                                { qr_button }
                            </p>
                        </div>
                    </div>
                </div>
                <div class="card-image has-text-centered">
                    <figure onclick={ trigger_event_cb } class="image w-75 is-square is-clickable is-inline-block">
                        <img class="box" src={ image_url } alt="Action image"/>
                    </figure>
                    <div class="content">{description}</div>
                </div>
            </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
