use data_url::{mime, DataUrl};
use std::{
    cell::RefCell,
    convert::TryInto,
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::{
    characters::CharacterQRJson,
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};
use js_sys::{ArrayBuffer, Function, Uint8Array};
use serde::Deserialize;
use uuid::Uuid;
use yew::{html, prelude::*};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub new_world: Callback<()>,
    pub story_name: String,
    pub character_scanned: Callback<(String, Uuid)>,
}

pub enum Msg {
    NewWorld,
    QRCodeScanShow,
    QRCodeScanned(String),
}

pub struct Intro {
    pub qr_scanner_character_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
}

impl Component for Intro {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewWorld => {
                ctx.props().new_world.emit(());
                true
            }
            Msg::QRCodeScanShow => {
                self.qr_scanner_character_callback
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRScannerMsg::Active(true));
                false
            }
            Msg::QRCodeScanned(content) => {
                match DataUrl::process(&content) {
                    Ok(data_url) => match data_url.decode_to_vec() {
                        Ok((data, _)) => {
                            match serde_json::from_slice::<CharacterQRJson>(&data[..]) {
                                Ok(character_json) => {
                                    let CharacterQRJson {
                                        character,
                                        world_id,
                                    } = character_json;
                                    log::debug!(
                                        "Joining world (id={:?}) as character {}",
                                        &world_id,
                                        &character
                                    );
                                    ctx.props().character_scanned.emit((character, world_id));
                                }
                                Err(err) => {
                                    log::warn!("Error while processing data: {:?}", err);
                                }
                            }
                        }
                        Err(err) => {
                            log::warn!("Failed to parse base64 {:?}", err);
                            // Retry to scan the image
                            self.qr_scanner_character_callback
                                .as_ref()
                                .borrow()
                                .clone()
                                .unwrap()
                                .send_message(QRScannerMsg::Active(true));
                        }
                    },
                    Err(err) => {
                        log::warn!("Can't process scanned data: {:?}", err);
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let qr_found_cb = link.callback(|string| Msg::QRCodeScanned(string));

        let new_world_cb = link.callback(|_| Msg::NewWorld);
        let show_qr_cb = link.callback(|_| Msg::QRCodeScanShow);
        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{ctx.props().story_name.clone()}</p>
                    </header>
                    <section class="modal-card-body">
                        <div class="columns is-flex-wrap-wrap w-100">
                            <figure class="image is-128x128 is-clickable box" onclick={new_world_cb}>
                                <img src="svgs/solid/plus-circle.svg"/>
                            </figure>
                            <figure class="image is-128x128 is-clickable box" onclick={show_qr_cb}>
                                <img src="svgs/solid/sign-in-alt.svg"/>
                            </figure>
                            <QRScanner
                              qr_found={qr_found_cb}
                              shared_scope={self.qr_scanner_character_callback.clone()}
                            />
                        </div>
                    </section>
                </div>
            </div>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            qr_scanner_character_callback: Rc::new(RefCell::new(None)),
        }
    }
}

impl Intro {}
