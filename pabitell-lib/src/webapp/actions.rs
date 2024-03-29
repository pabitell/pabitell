use data_url::DataUrl;
use geo::Point;
use serde_json::Value;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;
use yew::prelude::*;

use super::{
    action_event, action_item, action_join, characters,
    geo_navigator::{GeoNavigator, NavigationData},
    items,
    qrcode::{Msg as QRCodeMsg, QRCode},
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};
use crate::translations::get_message_global;

#[derive(Clone, Debug, Default, Properties)]
pub struct Props {
    pub lang: String,
    pub available_characters: Rc<Vec<Rc<characters::Character>>>,
    pub owned_items: Rc<Vec<Rc<items::Item>>>,
    pub character: Rc<Option<String>>,
    pub events: Vec<Rc<action_event::ActionEventItem>>,
    pub trigger_event_data: Callback<Value>,
    pub world_id: Uuid,
    pub actions_scope: Rc<RefCell<Option<html::Scope<Actions>>>>,
    pub finished: bool,
    pub nav_data: Vec<NavigationData>,
    pub position_reached_cb: Callback<(String, Point)>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang
            && self.available_characters == other.available_characters
            && self.owned_items == other.owned_items
            && self.character == other.character
            && self.events == other.events
            && self.trigger_event_data == other.trigger_event_data
            && self.world_id == other.world_id
            && self.finished == other.finished
            && self.nav_data == other.nav_data
    }
}

pub enum Msg {
    QRCodeScanShow,
    TriggerEventData(Rc<Vec<u8>>),
    QRCodeShow(Rc<Vec<u8>>),
    QRCodeHide,
    QRCodeScanned(String),
    QRCodeUseItemScanned(String, String),
    PositionReached(String, Point),
}

pub struct Actions {
    qr_code_scope: Rc<RefCell<Option<html::Scope<QRCode>>>>,
    qr_scanner_scope: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
}

impl Component for Actions {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().actions_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            qr_code_scope: Rc::new(RefCell::new(None)),
            qr_scanner_scope: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::QRCodeUseItemScanned(item, content) => {
                match DataUrl::process(&content) {
                    Ok(data_url) => match data_url.decode_to_vec() {
                        Ok((json_str, _)) => match serde_json::from_slice(&json_str[..]) {
                            Ok(json) => {
                                let mut value: Value = json;
                                value["item"] = Value::String(item);
                                ctx.props().trigger_event_data.emit(value);
                            }
                            Err(err) => {
                                log::warn!("Can't decode scanned data to json: {:?}", err);
                            }
                        },
                        Err(err) => {
                            log::warn!("Failed to parse base64 {:?}", err);
                        }
                    },
                    Err(err) => {
                        log::warn!("Can't process scanned data: {:?}", err);
                    }
                }
                false
            }
            Msg::QRCodeScanned(content) => {
                match DataUrl::process(&content) {
                    Ok(data_url) => match data_url.decode_to_vec() {
                        Ok((json_str, _)) => match serde_json::from_slice(&json_str[..]) {
                            Ok(json) => {
                                ctx.props().trigger_event_data.emit(json);
                            }
                            Err(err) => {
                                log::warn!("Can't decode scanned data to json: {:?}", err);
                            }
                        },
                        Err(err) => {
                            log::warn!("Failed to parse base64 {:?}", err);
                        }
                    },
                    Err(err) => {
                        log::warn!("Can't process scanned data: {:?}", err);
                    }
                }
                false
            }
            Msg::TriggerEventData(data) => {
                let data = serde_json::from_slice(&data).unwrap();
                ctx.props().trigger_event_data.emit(data);
                true
            }
            Msg::QRCodeShow(data) => {
                self.qr_code_scope
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRCodeMsg::Show(Some(data)));
                false
            }
            Msg::QRCodeHide => {
                self.qr_code_scope
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRCodeMsg::Show(None));
                false
            }
            Msg::QRCodeScanShow => {
                self.qr_scanner_scope
                    .as_ref()
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_message(QRScannerMsg::Active(true));
                false
            }
            Msg::PositionReached(character, point) => {
                ctx.props().position_reached_cb.emit((character, point));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let props = ctx.props();
        let qr_found_cb = link.callback(Msg::QRCodeScanned);
        let render_action = move |item: Rc<action_event::ActionEventItem>| {
            let event_data = item.data.clone();
            let cb = link
                .clone()
                .callback(move |_| Msg::TriggerEventData(event_data.clone()));
            let show_qr_cb = link.callback(Msg::QRCodeShow);

            html! {
                <action_event::ActionEvent {item} trigger_event_cb={cb} {show_qr_cb} />
            }
        };
        let cloned_link = ctx.link().clone();
        let lang = ctx.props().lang.clone();
        let geo_navs = |data: NavigationData| {
            let (character, scene_name, scene_title, destination) = data;
            let lang = lang.clone();
            let link = cloned_link.clone();

            let destination_cloned = destination.clone();
            if let Some(character_code) = character.code.as_ref() {
                let character_code = character_code.to_owned();
                let reached_cb = link.callback(move |_| {
                    Msg::PositionReached(character_code.clone(), destination_cloned)
                });
                html! {
                    <GeoNavigator
                      {lang}
                      {destination}
                      character={character.clone()}
                      reached={reached_cb}
                      {scene_name}
                      {scene_title}
                    />
                }
            } else {
                // don't show navigation for narrator
                html! {}
            }
        };
        let cloned_link = ctx.link().clone();
        let qr_scans = move |character: Rc<characters::Character>| {
            let scan_cb = cloned_link.callback(move |_| Msg::QRCodeScanShow);
            let qr_code_text = get_message_global("qr_code", &props.lang, None);
            let qr_code_scan_text = get_message_global("qr_code_scan", &props.lang, None);
            html! {
                <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                    <div class="card-content">
                        <div class="media">
                            <div class="media-left">
                                <figure class="image is-48x48">
                                    <img src="images/search.svg"/>
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
        let characters: Vec<_> = if !props.finished {
            props
                .available_characters
                .iter()
                .filter(|e| e.code.is_some() && (props.character == e.code))
                .collect()
        } else {
            vec![]
        };

        let joinable_characters: Vec<_> = if props.character.is_none() && !props.finished {
            ctx.props().available_characters.iter().collect()
        } else {
            vec![]
        };
        let events: Vec<_> = if let Some(character) = props.character.as_ref() {
            props
                .events
                .clone()
                .into_iter()
                .filter(|e| e.self_triggering && e.character.name.as_ref() == character)
                .collect()
        } else {
            props.events.clone().into_iter().collect()
        };

        let link = ctx.link().clone();
        let render_item = move |item: &Rc<items::Item>| {
            let use_item_scanned_cb =
                link.callback(|(item, data)| Msg::QRCodeUseItemScanned(item, data));

            let show_qr_cb = link.callback(Msg::QRCodeShow);
            let trigger_event_cb = link.callback(Msg::TriggerEventData);
            html! {
                <action_item::ActionItem
                  item={item.clone()}
                  item_used_event={use_item_scanned_cb}
                  lang={ctx.props().lang.clone()}
                  {show_qr_cb}
                  {trigger_event_cb}
                />
            }
        };

        let link = ctx.link().clone();
        let render_join = |character: &Rc<characters::Character>| {
            let world_id = props.world_id;
            let show_qr_cb = link.callback(Msg::QRCodeShow);
            html! {
                <action_join::ActionJoin
                  {show_qr_cb}
                  character={character.clone()}
                  {world_id}
                  lang={ctx.props().lang.clone()}
                />
            }
        };

        let items = props.owned_items.clone();

        let nav_data = ctx
            .props()
            .nav_data
            .clone()
            .into_iter()
            .filter(|(character, _, _, _)| {
                if let Some(character2) = ctx.props().character.as_ref() {
                    if let Some(code) = character.code.as_ref() {
                        code == character2
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

        let scanner_text = get_message_global("move_pick_or_use", &props.lang, None);

        html! {
            <section class="section is-flex">
                <div class="columns is-flex-wrap-wrap w-100">
                    { for nav_data.into_iter().map(move |e| geo_navs(e)) }
                    { for characters.clone().into_iter().map(|e| qr_scans(e.clone())) }
                    { for joinable_characters.iter().map(|e| render_join(e)) }
                    { for items.iter().map(render_item) }
                    <QRScanner
                      qr_found={qr_found_cb}
                      lang={ctx.props().lang.clone()}
                      shared_scope={self.qr_scanner_scope.clone()}
                      title={Rc::new(scanner_text)}
                    />
                    { for events.into_iter().map(render_action) }
                    <QRCode qr_code_scope={self.qr_code_scope.clone()}/>
                </div>
            </section>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().actions_scope.borrow_mut() = Some(ctx.link().clone());
        true
    }
}
