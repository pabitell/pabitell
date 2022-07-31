use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use data_url::DataUrl;
use js_sys::{Array, Uint8Array};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::{html, prelude::*, NodeRef};

use super::{
    characters, database,
    language_switch::LanguageSwitch,
    qrscanner::{Msg as QRScannerMsg, QRScanner},
};

use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, HtmlInputElement, Url};

use crate::translations;

#[derive(Clone, Debug, PartialEq)]
pub enum FailedLoadState {
    WrongJson,
    NotJson,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub new_world: Callback<()>,
    pub show_print: Callback<bool>,
    pub try_load_world_cb: Callback<Value>,
    pub load_failed_cb: Callback<FailedLoadState>,
    pub load_failed: Option<FailedLoadState>,
    pub set_language: Callback<String>,
    pub languages: Rc<Vec<String>>,
    pub name: String,
    pub available_characters: Rc<Vec<Rc<characters::Character>>>,
    pub story_name: String,
    pub story_detail: String,
    pub character_scanned: Callback<(Option<String>, bool, Uuid)>,
    pub lang: String,
}

pub enum Msg {
    NewWorld,
    LoadWorld(Value),
    QRCodeScanShow,
    QRCodeScanned(String),
    ShowPrint,
    UpdateWorlds(Vec<(DateTime<Utc>, usize, String, bool, Uuid, Value)>),
    WorldDelete(Uuid),
    WorldPicked(Uuid, String, bool),
    SetLanguage(String),
    DownloadWorld(Uuid, Value),
    ShowUpload,
    FileChanged,
    FileReadFailed,
}

pub struct Intro {
    pub link_ref: NodeRef,
    pub file_ref: NodeRef,
    pub qr_scanner_character_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
    pub worlds: Vec<(DateTime<Utc>, usize, String, bool, Uuid, Value)>,
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
            Msg::SetLanguage(lang) => {
                ctx.props().set_language.emit(lang);
                true
            }
            Msg::DownloadWorld(id, world) => {
                let world_data = world.to_string();

                // Input of Blob::new_with_u8_array_sequence_and_options is array of Uint8Array
                let array = Array::new_with_length(1);
                array.set(0, JsValue::from(Uint8Array::from(world_data.as_bytes())));

                let mut blob_props = BlobPropertyBag::new();
                blob_props.type_("application/json");

                let blob =
                    Blob::new_with_u8_array_sequence_and_options(&array, &blob_props).unwrap();

                // Prepare url
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let link = self.link_ref.cast::<HtmlAnchorElement>().unwrap();
                link.set_href(&url);
                link.set_download(&format!("{}.json", id));

                // Trigger file download
                link.click();

                // Cleanup
                Url::revoke_object_url(&url).unwrap();
                false
            }
            Msg::LoadWorld(world) => {
                ctx.props().try_load_world_cb.emit(world);
                true
            }
            Msg::ShowUpload => {
                let link = self.file_ref.cast::<HtmlInputElement>().unwrap();
                link.click();
                false
            }
            Msg::FileChanged => {
                let link = self.file_ref.cast::<HtmlInputElement>().unwrap();
                let files = link.files().unwrap();
                if files.length() == 0 {
                    return false;
                }
                let file = link.files().unwrap().item(0).unwrap();
                let promise = file.text();
                ctx.link().send_future(async move {
                    if let Ok(value) = JsFuture::from(promise).await {
                        if let Ok(data) = serde_json::from_str(&value.as_string().unwrap()) {
                            return Msg::LoadWorld(data);
                        }
                    }
                    Msg::FileReadFailed
                });
                false
            }
            Msg::FileReadFailed => {
                ctx.props().load_failed_cb.emit(FailedLoadState::NotJson);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props().clone();
        let lang = props.lang.to_owned();

        let qr_found_cb = link.callback(Msg::QRCodeScanned);
        let show_print_cb = link.callback(|_| Msg::ShowPrint);

        let new_world_cb = link.callback(|_| Msg::NewWorld);
        let show_qr_cb = link.callback(|_| Msg::QRCodeScanShow);
        let set_language_cb = link.callback(Msg::SetLanguage);

        let show_upload_cb = link.callback(|_| Msg::ShowUpload);
        let file_picked_cb = link.callback(|_| Msg::FileChanged);

        let table = if !self.worlds.is_empty() {
            let characters: HashMap<String, _> = props
                .available_characters
                .iter()
                .map(|e| (e.name.to_string(), e))
                .collect();
            let cloned_link = link.clone();
            let render_stored_world = move |time: DateTime<Utc>,
                                            events,
                                            character: String,
                                            fixed_character,
                                            id: Uuid,
                                            world: Value| {
                let local_time: DateTime<Local> = DateTime::from(time.to_owned());
                let character = characters.get(&character).unwrap();
                let characters::Character { name, icon, .. } = character.as_ref().clone();
                let restore_cb = cloned_link
                    .callback(move |_| Msg::WorldPicked(id, name.to_string(), fixed_character));
                let delete_cb = cloned_link.callback(move |_| Msg::WorldDelete(id));

                let download_cb =
                    cloned_link.callback(move |_| Msg::DownloadWorld(id, world.clone()));

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
                            <button class="button is-small is-info is-outlined mr-1 ml-1">
                                <span class="icon">
                                    <i class="fas fa-download" onclick={download_cb}></i>
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
                                |(time, events, character, fixed_character, id, world)|
                                render_stored_world(*time, events, character.clone(), *fixed_character, *id, world.to_owned())
                            )
                        }
                      </table>
                    </div>
                    <a style="display:none" ref={self.link_ref.clone()}></a>
                </div>
            }
        } else {
            html! {}
        };

        let wrong_json = translations::get_message_global("wrong_json", &lang, None);
        let not_json = translations::get_message_global("not_json", &lang, None);

        let load_error = match ctx.props().load_failed {
            Some(FailedLoadState::WrongJson) => {
                html! {
                    <div class="notification is-danger">
                        <span class="icon">
                            <i class="fas fa-exclamation"></i>
                        </span>
                        <span>{ wrong_json }</span>
                    </div>
                }
            }
            Some(FailedLoadState::NotJson) => {
                html! {
                    <div class="notification is-danger is-light">
                        <span class="icon">
                            <i class="fas fa-exclamation"></i>
                        </span>
                        <span>{ not_json }</span>
                    </div>
                }
            }
            None => html! {},
        };

        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <div class="modal-card-subtitle is-flex-shrink-1 mr-2">
                            <LanguageSwitch
                                languages={props.languages.clone()}
                                {set_language_cb}
                                lang={props.lang.clone()}
                            />
                        </div>
                        <div class="modal-card-title is-flex-shrink-1">
                            {ctx.props().story_name.clone()}
                        </div>
                    </header>
                    <section class="modal-card-body">
                        <div class="content">
                            <p>{ctx.props().story_detail.clone()}</p>
                            <QRScanner
                              qr_found={qr_found_cb}
                              lang={ctx.props().lang.clone()}
                              shared_scope={self.qr_scanner_character_callback.clone()}
                            />
                        </div>
                        {table}
                    </section>
                    <footer class="modal-card-foot is-justify-content-center">
                        <div class="content">
                        <div class="columns">
                            <div class="column is-justify-content-center is-flex">
                                <button class="button is-medium is-success is-outlined mx-1">
                                    <span class="icon">
                                        <i class="fas fa-plus-circle" onclick={new_world_cb}></i>
                                    </span>
                                </button>
                                <button class="button is-medium is-info is-outlined mx-1">
                                    <span class="icon">
                                        <i class="fas fa-upload" onclick={show_upload_cb}></i>
                                    </span>
                                </button>
                                <button class="button is-medium is-info is-outlined mx-1">
                                    <span class="icon">
                                        <i class="fas fa-sign-in-alt" onclick={show_qr_cb}></i>
                                    </span>
                                </button>
                                <button class="button is-medium is-outlined mx-1 is-dark is-hidden-touch" onclick={show_print_cb}>
                                    <span class="icon">
                                        <i class="fas fa-print"></i>
                                    </span>
                                </button>
                            </div>
                        </div>
                        <div class="columns">
                            <div class="column is-size-7">{ load_error }</div>
                        </div>
                        </div>
                    </footer>
                </div>
                <input
                  type="file"
                  ref={self.file_ref.clone()}
                  style="display:none"
                  onchange={file_picked_cb}
                  accepts="application/json"
                />
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
            link_ref: NodeRef::default(),
            file_ref: NodeRef::default(),
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
                            let count = data["event_count"].as_u64()?;
                            Some((last, count as usize, character, fixed_character, id, data))
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
