use data_url::DataUrl;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;
use yew::{html, prelude::*};

use super::{
    characters,
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub new_world: Callback<()>,
    pub show_print: Callback<bool>,
    pub story_name: String,
    pub story_detail: String,
    pub character_scanned: Callback<(Option<String>, Uuid)>,
}

pub enum Msg {
    NewWorld,
    QRCodeScanShow,
    QRCodeScanned(String),
    ShowPrint,
}

pub struct Intro {
    pub qr_scanner_character_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
}

impl Component for Intro {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowPrint => {
                ctx.props().show_print.emit(true);
                false
            }
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
                            match serde_json::from_slice::<characters::CharacterQRJson>(&data[..]) {
                                Ok(character_json) => {
                                    let characters::CharacterQRJson {
                                        character,
                                        world_id,
                                    } = character_json;
                                    log::debug!(
                                        "Joining world (id={:?}) as character {:?}",
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
        let show_print_cb = link.callback(|_| Msg::ShowPrint);

        let new_world_cb = link.callback(|_| Msg::NewWorld);
        let show_qr_cb = link.callback(|_| Msg::QRCodeScanShow);
        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title is-flex-shrink-1">
                            {ctx.props().story_name.clone()}
                        </p>
                    </header>
                    <section class="modal-card-body">
                        <div class="content">
                            <p>{ctx.props().story_detail.clone()}</p>
                            <QRScanner
                              qr_found={qr_found_cb}
                              shared_scope={self.qr_scanner_character_callback.clone()}
                            />
                        </div>
                    </section>
                    <footer class="modal-card-foot is-justify-content-center">
                        <button class="button is-medium is-success is-outlined">
                            <span class="icon">
                                <i class="fas fa-plus-circle" onclick={new_world_cb}></i>
                            </span>
                        </button>
                        <button class="button is-medium is-info is-outlined">
                            <span class="icon">
                                <i class="fas fa-sign-in-alt" onclick={show_qr_cb}></i>
                            </span>
                        </button>
                        <button class="button is-medium is-outlined is-dark is-hidden-touch" onclick={show_print_cb}>
                            <span class="icon">
                                <i class="fas fa-print"></i>
                            </span>
                        </button>
                    </footer>
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
