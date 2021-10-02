use data_url::{mime, DataUrl};
use pabitell_lib::{Character, Description, World, WorldBuilder};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

use super::{
    action_event, action_item, characters, items,
    qrcode::{Msg as QRCodeMsg, QRCode},
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {
    pub lang: String,
    pub available_characters: Rc<Vec<Rc<characters::Character>>>,
    pub owned_items: Rc<Vec<Rc<items::Item>>>,
    pub selected_character: Rc<Option<String>>,
    pub events: Vec<Rc<action_event::ActionEventItem>>,
    pub trigger_event: Callback<usize>,
    pub trigger_scanned_event: Callback<Value>,
}

pub enum Msg {
    QRCodeScanShow,
    TriggerEvent(usize),
    QRCodeShow(String),
    QRCodeScanned(String),
}

pub struct Actions {
    qr_callabacks: RefCell<HashMap<String, Rc<RefCell<Option<html::Scope<QRCode>>>>>>,
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
                match DataUrl::process(&content) {
                    Ok(data_url) => match data_url.decode_to_vec() {
                        Ok((json_str, _)) => match serde_json::from_slice(&json_str[..]) {
                            Ok(json) => {
                                ctx.props().trigger_scanned_event.emit(json);
                            }
                            Err(err) => {
                                log::warn!("Can't decode scanned data to json: {:?}", err);
                            }
                        },
                        Err(err) => {
                            log::warn!("Failed to parse base64 {:?}", err);
                            // Retry to scan the image
                            // note that bardecoder doesn't to
                            // error correction quire well
                            // So an invalid data may be loaded
                            self.qr_scanner_callback
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
            Msg::TriggerEvent(idx) => {
                ctx.props().trigger_event.emit(idx);
                true
            }
            Msg::QRCodeShow(data) => {
                self.qr_callabacks
                    .clone()
                    .borrow()
                    .get(&data)
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
        let qr_found_cb = link.callback(move |string| Msg::QRCodeScanned(string));
        let render_action = move |item: Rc<action_event::ActionEventItem>| {
            let idx = item.idx;
            let cb = link.clone().callback(move |_| Msg::TriggerEvent(idx));

            html! {
                <action_event::ActionEvent {item} trigger_event_cb={cb} />
            }
        };
        let cloned_link = ctx.link().clone();
        let qr_scans = move |character: Rc<characters::Character>| {
            let scan_cb = cloned_link.callback(move |_| Msg::QRCodeScanShow);
            let qr_code_text = get_message("qr_code", &ctx.props().lang, None);
            let qr_code_scan_text = get_message("qr_code_scan", &ctx.props().lang, None);
            html! {
                <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                    <div class="card-content">
                        <div class="media">
                            <div class="media-left">
                                <figure class="image is-48x48">
                                    <img src="svgs/solid/search.svg"/>
                                </figure>
                            </div>
                            <div class="media-content">
                                <p class="title is-4">{character.short.clone()}</p>
                                <p class="subtitle is-6">{qr_code_text}</p>
                            </div>
                        </div>
                    </div>
                    <div class="card-image has-text-centered">
                        <figure onclick={ scan_cb } class="image is-clickable is-square w-75 is-inline-block box">
                            <img class="box" src="images/qrcode.svg" alt="QR code"/>
                        </figure>
                        <div class="content">{qr_code_scan_text}</div>
                    </div>
                </div>
            }
        };
        let characters: Vec<_> = ctx
            .props()
            .available_characters
            .iter()
            .filter(|e| e.code.is_some() && (ctx.props().selected_character == e.code))
            .collect();

        let events: Vec<_> = if ctx.props().selected_character.is_none() {
            ctx.props().events.clone().into_iter().collect()
        } else {
            vec![]
        };

        let render_item = |item: &Rc<items::Item>| {
            html! {
                <action_item::ActionItem item={item.clone()}/>
            }
        };

        let items = ctx.props().owned_items.clone();

        html! {
            <section class="section is-flex">
                <div class="columns is-flex-wrap-wrap w-100">
                    { for characters.into_iter().map(|e| qr_scans(e.clone())) }
                    { for items.iter().map(render_item) }
                    <QRScanner qr_found={qr_found_cb} shared_scope={self.qr_scanner_callback.clone()}>
                    </QRScanner>
                    { for events.into_iter().map(render_action) }
                </div>
            </section>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
