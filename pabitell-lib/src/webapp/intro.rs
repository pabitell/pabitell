use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use data_url::DataUrl;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::{html, prelude::*};

use super::{
    characters, database,
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

use crate::translations;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub new_world: Callback<()>,
    pub show_print: Callback<bool>,
    pub name: String,
    pub available_characters: Rc<Vec<Rc<characters::Character>>>,
    pub story_name: String,
    pub story_detail: String,
    pub character_scanned: Callback<(Option<String>, bool, Uuid)>,
    pub lang: String,
}

pub enum Msg {
    NewWorld,
    QRCodeScanShow,
    QRCodeScanned(String),
    ShowPrint,
    UpdateWorlds(Vec<(DateTime<Utc>, usize, String, bool, Uuid)>),
    WorldDelete(Uuid),
    WorldPicked(Uuid, String, bool),
}

pub struct Intro {
    pub qr_scanner_character_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
    pub worlds: Vec<(DateTime<Utc>, usize, String, bool, Uuid)>,
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
            Msg::UpdateWorlds(worlds) => {
                let len = worlds.len();
                log::debug!("Worlds obtained {len} world(s).");
                self.worlds = worlds;
                true
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
                                    let fixed_character = character.is_some();
                                    ctx.props().character_scanned.emit((
                                        character,
                                        fixed_character,
                                        world_id,
                                    ));
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
            Msg::WorldPicked(id, character, fixed_character) => {
                ctx.props().character_scanned.emit((
                    if character == "narrator" {
                        None
                    } else {
                        Some(character)
                    },
                    fixed_character,
                    id,
                ));
                true
            }
            Msg::WorldDelete(id) => {
                let name = ctx.props().name.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let db = database::init_database(&name).await;
                    if database::del_world(&db, &id).await.is_ok() {
                        Self::update_worlds(name, link);
                    }
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props().clone();
        let lang = props.lang.to_owned();

        let qr_found_cb = link.callback(|string| Msg::QRCodeScanned(string));
        let show_print_cb = link.callback(|_| Msg::ShowPrint);

        let new_world_cb = link.callback(|_| Msg::NewWorld);
        let show_qr_cb = link.callback(|_| Msg::QRCodeScanShow);

        let table = if !self.worlds.is_empty() {
            let characters: HashMap<String, _> = props
                .available_characters
                .iter()
                .map(|e| (e.name.to_string(), e))
                .collect();
            let cloned_link = link.clone();
            let render_stored_world =
                |time: DateTime<Utc>, events, character: String, fixed_character, id: Uuid| {
                    let local_time: DateTime<Local> = DateTime::from(time.to_owned());
                    let character = characters.get(&character).unwrap();
                    let characters::Character { name, icon, .. } = character.as_ref().clone();
                    let restore_cb = cloned_link.callback(move |_| {
                        Msg::WorldPicked(id.clone(), name.to_string(), fixed_character)
                    });
                    let delete_cb = cloned_link.callback(move |_| Msg::WorldDelete(id));

                    html! {
                        <tr>
                            <td>
                                {{
                                    format!(
                                        "{:04}-{:02}-{:02} {:02}:{:02}",
                                        local_time.year(),
                                        local_time.month(),
                                        local_time.day(),
                                        local_time.hour(),
                                        local_time.minute(),
                                    )
                                }}
                            </td>
                            <td class="has-text-centered">{{ events }}</td>
                            <td class="has-text-centered">
                                <span class="icon is-small">
                                    <i class={ icon.to_string() }></i>
                                </span>
                            </td>
                            <td class="has-text-centered">
                                <button class="button is-small is-info is-outlined mr-1">
                                    <span class="icon">
                                        <i class="fas fa-sign-in-alt" onclick={restore_cb}></i>
                                    </span>
                                </button>
                                <button class="button is-small is-danger is-outlined ml-1">
                                    <span class="icon">
                                        <i class="fas fa-trash-alt" onclick={delete_cb}></i>
                                    </span>
                                </button>
                            </td>
                        </tr>
                    }
                };

            let last_stories = translations::get_message_global("last_stories", &lang, None);
            html! {
                <div class="content">
                    <h5>{{ last_stories }}</h5>
                    <div class="table-container">
                      <table class="table is-bordered">
                        {
                            for self.worlds.iter().map(
                                |(time, events, character, fixed_character, id)|
                                render_stored_world(time.clone(), events, character.clone(), *fixed_character, id.clone())
                            )
                        }
                      </table>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

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
                        {table}
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        Intro::update_worlds(ctx.props().name.clone(), ctx.link().clone());
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        Intro::update_worlds(ctx.props().name.clone(), ctx.link().clone());
        Self {
            qr_scanner_character_callback: Rc::new(RefCell::new(None)),
            worlds: vec![],
        }
    }
}

impl Intro {
    fn update_worlds(name: String, link: html::Scope<Self>) {
        link.send_future(async move {
            let db = database::init_database(&name).await;
            match database::get_worlds(&db).await {
                Ok(worlds) => Msg::UpdateWorlds(
                    worlds
                        .into_iter()
                        .filter_map(|(last, id, character, fixed_character, data)| {
                            let count = if let Some(count) = data["event_count"].as_u64() {
                                count
                            } else {
                                return None;
                            };
                            Some((last, count as usize, character, fixed_character, id.clone()))
                        })
                        .collect(),
                ),
                Err(err) => {
                    log::error!("Failed to get worlds {}", err);
                    Msg::UpdateWorlds(vec![])
                }
            }
        });
    }
}
