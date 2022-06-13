use super::{
    items,
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};
use std::{cell::RefCell, rc::Rc};
use yew::{html, prelude::*};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub item: Rc<items::Item>,
    pub item_used_event: Callback<(String, String)>,
    pub show_qr_cb: Callback<Rc<Vec<u8>>>,
    pub trigger_event_cb: Callback<Rc<Vec<u8>>>,
    pub lang: String,
}

pub struct ActionItem {
    pub qr_scanner_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
}

pub enum Msg {
    ShowQRCode,
    TriggerEventByScan(String),
    QRCodeScanShow,
    TriggerEventByClick(Rc<Vec<u8>>),
}

impl Component for ActionItem {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::ShowQRCode => {
                log::info!("QR active");
                let item = ctx.props().item.clone();
                if let Some(give_data) = item.give_data.as_ref() {
                    props.show_qr_cb.emit(give_data.clone());
                }
            }
            Msg::TriggerEventByScan(data) => {
                props
                    .item_used_event
                    .emit((props.item.code.to_string(), data));
            }
            Msg::QRCodeScanShow => {
                self.qr_scanner_callback
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRScannerMsg::Active(true));
            }
            Msg::TriggerEventByClick(data) => {
                props.trigger_event_cb.emit(data);
            }
        }
        false
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            qr_scanner_callback: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = ctx.props().item.clone();
        let link = ctx.link().clone();

        let show_qr = link.callback(|_| Msg::ShowQRCode);
        let qr_found_cb = link.callback(move |string| Msg::TriggerEventByScan(string));
        let scan_cb = link.callback(move |_| Msg::QRCodeScanShow);
        let scan_html = if item.scan {
            html! {
                <button class="button is-large card-footer-item" onclick={ scan_cb.clone() }>
                    <i class="fas fa-qrcode"></i>
                </button>
            }
        } else {
            html! {}
        };
        let use_html = if let Some(use_data) = item.use_data.clone() {
            let use_cb = link.callback(move |_| Msg::TriggerEventByClick(use_data.clone()));
            html! {
                <button class="button is-large card-footer-item" onclick={ use_cb }>
                    <i class="fas fa-cogs"></i>
                </button>
            }
        } else {
            html! {}
        };
        let give_html = if item.give_data.is_some() {
            html! {
                <button class="button is-large card-footer-item" onclick={ show_qr.clone() }>
                    <i class="fas fa-gift"></i>
                </button>
            }
        } else {
            html! {}
        };

        let big_picture = match item.default {
            items::DefaultAction::Give => {
                html! {
                    <figure class="image is-square w-75 is-inline-block is-clickable box"  onclick={ show_qr }>
                        <img class="box" src={ item.image_url.clone() }/>
                    </figure>
                }
            }
            items::DefaultAction::Scan => {
                html! {
                    <figure class="image is-square w-75 is-inline-block is-clickable box" onclick={scan_cb.clone()}>
                        <img src={ item.image_url.clone() }/>
                    </figure>
                }
            }
            items::DefaultAction::Use => {
                if let Some(use_data) = item.use_data.clone() {
                    let use_cb = link.callback(move |_| Msg::TriggerEventByClick(use_data.clone()));
                    html! {
                        <figure class="image is-square w-75 is-inline-block is-clickable box" onclick={use_cb} >
                            <img src={ item.image_url.clone() }/>
                        </figure>
                    }
                } else {
                    html! {
                        <figure class="image is-square w-75 is-inline-block box">
                            <img src={ item.image_url.clone() }/>
                        </figure>
                    }
                }
            }
        };

        html! {
            <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-48x48">
                                <img src="images/backpack.svg" alt="Backpack"/>
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-4">{item.short.clone()}</p>
                            <p class="subtitle is-6"></p>
                        </div>
                    </div>
                </div>
                <div class="card-image has-text-centered">
                    { big_picture }
                </div>
                <div class="card-content">
                    <div class="content">{item.long.clone()}</div>
                    <QRScanner
                      lang={ctx.props().lang.clone()}
                      qr_found={qr_found_cb}
                      shared_scope={self.qr_scanner_callback.clone()}
                    />
                </div>
                <footer class="card-footer">
                    { scan_html }
                    { use_html }
                    { give_html }
                </footer>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
