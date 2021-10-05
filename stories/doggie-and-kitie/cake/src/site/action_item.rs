use serde_json::Value;
use std::{cell::RefCell, rc::Rc};
use yew::{html, prelude::*, web_sys::Element};

use super::{
    characters, items, qrcode,
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub item: Rc<items::Item>,
    pub item_used_event: Callback<(String, String)>,
}

pub struct ActionItem {
    pub qr_scope: Rc<RefCell<Option<html::Scope<qrcode::QRCode>>>>,
    pub qr_scanner_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
}

pub enum Msg {
    ShowQRCode,
    QRCodeScanned(String),
    QRCodeScanShow,
}

impl Component for ActionItem {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
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
            Msg::QRCodeScanned(data) => {
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
        }
        false
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            qr_scanner_callback: Rc::new(RefCell::new(None)),
            qr_scope: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = ctx.props().item.clone();

        let data = item.data.clone();
        let link = ctx.link().clone();

        let show_qr = link.callback(|_| Msg::ShowQRCode);
        let qr_found_cb = link.callback(move |string| Msg::QRCodeScanned(string));
        let scan_cb = link.callback(move |_| Msg::QRCodeScanShow);

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
                    <figure class="image is-square w-75 is-inline-block box"  >
                        <img src={ item.image_url.clone() }/>
                    </figure>
                </div>
                <div class="card-content">
                    <div class="content">{item.long.clone()}</div>
                    <qrcode::QRCode {data} shared_scope={self.qr_scope.clone()} />
                    <QRScanner qr_found={qr_found_cb} shared_scope={self.qr_scanner_callback.clone()} />
                </div>
                <footer class="card-footer">
                    <button class="button is-large card-footer-item" onclick={ scan_cb }>
                        <i class="fas fa-cogs"></i>
                    </button>
                    <button class="button is-large card-footer-item" onclick={ show_qr }>
                        <i class="fas fa-gift"></i>
                    </button>
                </footer>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
