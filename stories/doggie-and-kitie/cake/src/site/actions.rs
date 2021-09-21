use data_url::{mime, DataUrl};
use pabitell_lib::{Character, Description, World, WorldBuilder};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

use super::{
    action, characters,
    qrcode::{Msg as QRCodeMsg, QRCode},
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {
    pub events: Vec<Rc<action::EventActionItem>>,
    pub trigger_event: Callback<usize>,
    pub trigger_scanned_event: Callback<Value>,
}

pub enum Msg {
    QRCodeScanShow,
    TriggerEvent(usize),
    QRCodeShow(usize),
    QRCodeScanned(String),
}

pub struct Actions {
    qr_callabacks: RefCell<HashMap<usize, Rc<RefCell<Option<html::Scope<QRCode>>>>>>,
    qr_scanner_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
}

impl Component for Actions {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            qr_scanner_callback: Rc::new(RefCell::new(None)),
            qr_callabacks: RefCell::new(HashMap::new()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::QRCodeScanned(content) => {
                log::debug!("{}", content);
                match DataUrl::process(&content) {
                    Ok(data_url) => {
                        let (json_str, _) = data_url.decode_to_vec().unwrap(); // TODO invalid base64 errors
                        match serde_json::from_slice(&json_str[..]) {
                            Ok(json) => {
                                ctx.props().trigger_scanned_event.emit(json);
                            }
                            Err(err) => {
                                log::warn!("Can't decode scanned data to json: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!("Can't process scanned data: {:?}", err);
                    }
                }
                false
            }
            Msg::TriggerEvent(idx) => {
                ctx.props().trigger_event.emit(idx);
                true
            }
            Msg::QRCodeShow(idx) => {
                self.qr_callabacks
                    .clone()
                    .borrow()
                    .get(&idx)
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRCodeMsg::Active(true));
                false
            }
            Msg::QRCodeScanShow => {
                self.qr_scanner_callback
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRScannerMsg::Active(true));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let scan_cb = link.callback(move |_| Msg::QRCodeScanShow);
        let qr_found_cb = link.callback(move |string| Msg::QRCodeScanned(string));
        let render_action = move |item: Rc<action::EventActionItem>| {
            let idx = item.idx;
            let cb = link.callback(move |_| Msg::TriggerEvent(idx));

            html! {
                <action::Action {item} trigger_event_cb={cb} />
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
                            <QRScanner qr_found={qr_found_cb} shared_scope={self.qr_scanner_callback.clone()}>
                            </QRScanner>
                        </div>
                    </div>
                    { for ctx.props().events.clone().into_iter().map(render_action) }
                </div>
            </section>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
