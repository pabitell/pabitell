use chrono::{DateTime, Datelike, Local, Timelike};
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
    database::StoredWorld,
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
    pub world_version: usize,
}

pub enum Msg {
    NewWorld,
    LoadWorld(Value),
    QRCodeScanShow,
    QRCodeScanned(String),
    ShowPrint,
    UpdateWorlds(Vec<database::StoredWorld>),
    WorldDelete(Uuid),
    WorldPicked(Uuid, String, bool),
    SetLanguage(String),
    DownloadWorld(Uuid, Value),
    ShowUpload,
    FileChanged,
    FileReadFailed,
    UpdateWorldName(Uuid, String),
    EditableNameToggle(Uuid, NodeRef),
}

pub struct Intro {
    pub link_ref: NodeRef,
    pub file_ref: NodeRef,
    pub qr_scanner_character_callback: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
    pub worlds: Vec<database::StoredWorld>,
    pub editable_worlds: HashMap<Uuid, NodeRef>,
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
                let version = ctx.props().world_version;
                spawn_local(async move {
                    let db = database::init_database(&name).await;
                    if database::del_world(&db, &id).await.is_ok() {
                        Self::update_worlds(name, link, version);
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
                link.set_download(&format!(
                    "{}-V{}-{}.json",
                    ctx.props().name,
                    ctx.props().world_version,
                    id
                ));

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
            Msg::UpdateWorldName(id, name) => {
                let link = ctx.link().clone();
                let story_name = ctx.props().name.clone();
                let world_version = ctx.props().world_version.clone();

                spawn_local(async move {
                    let db = database::init_database(&story_name).await;
                    match database::get_world(&db, &id).await {
                        Ok(Some(mut world)) => {
                            world.name = Some(name);
                            if let Err(err) = database::put_world(&db, world).await {
                                log::error!("Failed to put world {}: {}", id, err);
                            } else {
                                Self::update_worlds(story_name, link, world_version);
                            }
                        }
                        Ok(None) => {
                            log::warn!("World {} not found", id);
                        }
                        Err(err) => {
                            log::error!("Failed to get world {}: {}", id, err);
                        }
                    }
                });
                false
            }
            Msg::EditableNameToggle(uuid, name_input_ref) => {
                if self.editable_worlds.remove(&uuid).is_none() {
                    self.editable_worlds.insert(uuid.clone(), name_input_ref);
                } else {
                    let new_name = name_input_ref.cast::<HtmlInputElement>().unwrap().value();
                    ctx.link()
                        .send_message(Msg::UpdateWorldName(uuid, new_name));
                }
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
            let render_stored_world = |stored_world: database::StoredWorld| {
                let local_time: DateTime<Local> = DateTime::from(stored_world.last.to_owned());
                let character = characters
                    .get(
                        &stored_world
                            .character
                            .clone()
                            .unwrap_or_else(|| "narrator".to_string()),
                    )
                    .unwrap();
                let characters::Character { name, icon, .. } = character.as_ref().clone();
                let world_id = stored_world.id.clone();
                let world_id_cloned = world_id.clone();
                let name_input_ref = NodeRef::default();
                let name_input_ref_cloned = name_input_ref.clone();
                let toggle_editable = link.callback(move |_| {
                    Msg::EditableNameToggle(world_id_cloned, name_input_ref_cloned.clone())
                });
                let name_editable = self.editable_worlds.contains_key(&world_id);
                let name_editable_classes = if name_editable {
                    classes!("button", "is-small", "is-info")
                } else {
                    classes!("button", "is-small", "is-info", "is-outlined")
                };
                let events = stored_world.data["event_count"].as_u64().unwrap();
                let restore_cb = cloned_link.callback(move |_| {
                    Msg::WorldPicked(world_id, name.to_string(), stored_world.fixed_character)
                });
                let delete_cb =
                    cloned_link.callback(move |_| Msg::WorldDelete(stored_world.id.clone()));

                let download_cb = cloned_link.callback(move |_| {
                    Msg::DownloadWorld(stored_world.id.clone(), stored_world.data.clone())
                });

                html! {
                    <tr>
                        <td class="is-vcentered">
                            <span class="is-size-7">
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
                            </span>
                            <div class="field has-addons">
                                <div class="control">
                                    <input
                                      class="input is-small"
                                      type="text"
                                      ref={name_input_ref}
                                      readonly={!name_editable}
                                      value={stored_world.name.unwrap_or_default()}
                                    />
                                </div>
                                <div class="control">
                                    <a
                                      class={ name_editable_classes }
                                      onclick={ toggle_editable }
                                    >
                                        <span class="icon">
                                            <i class="fas fa-edit"></i>
                                        </span>
                                    </a>
                                </div>
                            </div>
                        </td>
                        <td class="has-text-centered is-vcentered">{{ events }}</td>
                        <td class="has-text-centered is-vcentered">
                            <span class="icon is-small">
                                <i class={ icon.to_string() }></i>
                            </span>
                        </td>
                        <td class="has-text-centered is-vcentered">
                            <div class="columns p-1">
                                <div class="column m-0 p-0 is-12-mobile">
                            <button class="button is-small is-info is-outlined">
                                <span class="icon">
                                    <i class="fas fa-sign-in-alt" onclick={restore_cb}></i>
                                </span>
                            </button>
                                </div>
                                <div class="column m-0 p-0 is-12-mobile">
                            <button class="button is-small is-info is-outlined">
                                <span class="icon">
                                    <i class="fas fa-download" onclick={download_cb}></i>
                                </span>
                            </button>
                                </div>
                                <div class="column m-0 p-0 is-12-mobile">
                            <button class="button is-small is-danger is-outlined">
                                <span class="icon">
                                    <i class="fas fa-trash-alt" onclick={delete_cb}></i>
                                </span>
                            </button>
                                </div>
                            </div>
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
                            for self.worlds.clone().into_iter().map(render_stored_world)
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
        Self::update_worlds(
            ctx.props().name.clone(),
            ctx.link().clone(),
            ctx.props().world_version,
        );
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self::update_worlds(
            ctx.props().name.clone(),
            ctx.link().clone(),
            ctx.props().world_version,
        );
        Self {
            link_ref: NodeRef::default(),
            file_ref: NodeRef::default(),
            qr_scanner_character_callback: Rc::new(RefCell::new(None)),
            worlds: vec![],
            editable_worlds: HashMap::new(),
        }
    }
}

impl Intro {
    async fn get_worlds(
        name: String,
        world_version: usize,
    ) -> Result<Vec<StoredWorld>, rexie::Error> {
        let db = database::init_database(&name).await;
        database::get_worlds(&db).await.map(|e| {
            e.into_iter()
                .filter(|i| i.version == world_version)
                .collect()
        })
    }

    fn update_worlds(name: String, link: html::Scope<Self>, world_version: usize) {
        link.send_future(async move {
            match Self::get_worlds(name, world_version).await {
                Ok(worlds) => Msg::UpdateWorlds(worlds),
                Err(err) => {
                    log::error!("Failed to get worlds {}", err);
                    Msg::UpdateWorlds(vec![])
                }
            }
        });
    }
}
