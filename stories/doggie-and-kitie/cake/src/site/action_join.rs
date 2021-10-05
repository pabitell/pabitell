use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;
use yew::{html, prelude::*, web_sys::Element};

use super::{characters, qrcode};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub character: Rc<characters::Character>,
    pub world_id: Uuid,
}

pub struct ActionJoin {
    pub qr_scope: Rc<RefCell<Option<html::Scope<qrcode::QRCode>>>>,
}

pub enum Msg {
    ShowQRCode,
}

impl Component for ActionJoin {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowQRCode => {
                log::info!("QR active");
                self.qr_scope
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(qrcode::Msg::Active(true));
            }
        }
        false
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            qr_scope: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let character = ctx.props().character.clone();
        let character_code = character.code.as_ref().as_ref().unwrap().to_string();
        let character_url = character.character_url.to_string();

        let data = characters::CharacterQRJson::new(character_code, ctx.props().world_id.clone());
        let data = Rc::new(serde_json::to_vec(&data).unwrap());

        let onclick = ctx.link().callback(|_| Msg::ShowQRCode);

        html! {
            <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-48x48">
                                <img src={character_url} />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-4">{character.short.clone()}</p>
                            <p class="subtitle is-6"></p>
                        </div>
                    </div>
                </div>
                <div class="card-image has-text-centered">
                    <figure class="image is-clickable is-square w-75 is-inline-block box" {onclick} >
                        <img class="box" src={"svgs/solid/sign-in-alt.svg"}/>
                    </figure>
                    <qrcode::QRCode {data} shared_scope={self.qr_scope.clone()} />
                    <div class="content"></div>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
