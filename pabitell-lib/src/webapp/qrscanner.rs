use std::{cell::RefCell, rc::Rc};

use gloo::{
    storage::{self, Storage},
    timers,
};
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, HtmlVideoElement, ImageData,
    MediaDeviceInfo, MediaDeviceKind, MediaStream, MediaStreamConstraints, MediaStreamTrack,
    MouseEvent,
};
use yew::{html, prelude::*};

use crate::translations;

use super::jsQR::js_qr;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub qr_found: Callback<String>,
    pub shared_scope: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
    pub lang: String,
    pub title: Rc<String>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, rhs: &Self) -> bool {
        self.qr_found == rhs.qr_found
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    icon: String,
    label: String,
    device_id: String,
    disabled: bool,
}

pub struct QRScanner {
    cameras_loaded: bool,
    cameras: Vec<Camera>,
    active: bool,
    current_device_id: Option<String>,
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    input_ref: NodeRef,
    interval: Option<timers::callback::Interval>,
    camera_available: bool,
    show_text_input: bool,
}

pub enum Msg {
    Active(bool),
    CamerasLoaded(Vec<Camera>),
    Picture(ImageData),
    TextInput(String),
    SwitchCamera(Option<String>),
    CameraSwitched,
    CameraNotAvailable,
    ToggleTextInput,
}

impl Component for QRScanner {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Active(active) => {
                self.active = active;
                if !active {
                    if let Some(interval) = self.interval.take() {
                        interval.cancel();
                        self.turn_off_camera();
                    }
                } else {
                    // restore previous view on re-open
                    if let Some(device_id) = self.current_device_id.clone() {
                        // This needs to be send as future - to make sure
                        // that underlaying DOM is properly initialized
                        ctx.link()
                            .send_future(async move { Msg::SwitchCamera(Some(device_id)) });
                    }
                }
                true
            }
            Msg::CamerasLoaded(cameras) => {
                self.cameras = cameras;
                if !self.cameras.is_empty() {
                    self.cameras_loaded = true;

                    let device_id = if let Ok(device_id) = storage::SessionStorage::get("device_id")
                    {
                        // Check whether stored id is in list
                        if self.cameras.iter().any(|e| e.device_id == device_id) {
                            device_id
                        } else {
                            // Existing device id is missing -> pick first
                            self.cameras[0].device_id.clone()
                        }
                    } else {
                        // Pick first camera
                        self.cameras[0].device_id.clone()
                    };

                    // Send message to start streaming
                    ctx.link().send_message(Msg::SwitchCamera(Some(device_id)));
                }
                true
            }
            Msg::Picture(data) => {
                let width = data.width();
                let height = data.height();
                let input = data.data();

                let res = js_qr(
                    input,
                    width,
                    height,
                    <JsValue as JsValueSerdeExt>::from_serde(&serde_json::json!({})).unwrap(),
                );
                if let Some(qr_data) = res {
                    ctx.props().qr_found.emit(qr_data);
                    if let Some(interval) = self.interval.take() {
                        interval.cancel();
                    }
                    self.active = false;
                    true
                } else {
                    false
                }
            }
            Msg::TextInput(text) => {
                ctx.props().qr_found.emit(text);
                if let Some(interval) = self.interval.take() {
                    interval.cancel();
                }
                let input = self.input_ref.clone().cast::<HtmlInputElement>().unwrap();

                // Reset state
                input.set_value("");
                self.show_text_input = false;
                self.active = false;

                true
            }
            Msg::SwitchCamera(device_id) => {
                self.camera_available = true;
                // Store device in local storage
                if let Some(device_id) = device_id.as_ref() {
                    storage::SessionStorage::set(&"device_id", device_id).unwrap();
                } else {
                    storage::SessionStorage::delete(&"device_id");
                }

                let window = web_sys::window().unwrap();
                // prepare js instances
                let navigator = window.navigator();
                let media_devices = navigator.media_devices().unwrap();

                // Turn camera off
                self.turn_off_camera();

                let video = self.video_ref.cast::<HtmlVideoElement>().unwrap();
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                if let Some(device_id) = device_id.clone() {
                    // Turn camera on
                    ctx.link().send_future(async move {
                        let mut constraints = MediaStreamConstraints::new();
                        constraints.video(
                            &<JsValue as JsValueSerdeExt>::from_serde(
                                &serde_json::json!({"deviceId": {"exact": device_id}}),
                            )
                            .unwrap(),
                        );
                        let stream_fut =
                            media_devices.get_user_media_with_constraints(&constraints);

                        if let Ok(stream_js) = JsFuture::from(stream_fut.unwrap()).await {
                            let media_stream = MediaStream::from(stream_js);

                            video.set_src_object(Some(&media_stream));
                            JsFuture::from(video.play().unwrap()).await.unwrap();

                            canvas.set_width(video.video_width());
                            canvas.set_height(video.video_height());
                            Msg::CameraSwitched
                        } else {
                            Msg::CameraNotAvailable
                        }
                    });
                }
                self.current_device_id = device_id;
                true
            }

            Msg::CameraSwitched => {
                self.prepare_interval(ctx);
                false
            }
            Msg::CameraNotAvailable => {
                self.camera_available = false;
                true
            }
            Msg::ToggleTextInput => {
                self.show_text_input = !self.show_text_input;
                true
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            active: false,
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            interval: None,
            cameras_loaded: false,
            cameras: vec![Camera {
                icon: "fas fa-spinner fa-spin".into(),
                label: "".into(),
                device_id: "".into(),
                disabled: true,
            }],
            current_device_id: None,
            camera_available: true,
            show_text_input: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_cb = ctx.link().callback(|_| Msg::Active(false));
        let classes = if self.active {
            classes!("modal", "is-active")
        } else {
            classes!("modal")
        };

        html! {
            <div class={classes}>
                <div class="modal-background"></div>
                <div class="modal-content has-text-centered">
                    <div class="content has-text-white has-text-weight-bold">
                        <small>{ ctx.props().title.clone() }</small>
                    </div>
                { self.view_cameras(ctx) }
                { self.video_view(ctx) }
                </div>
                { self.view_text_input(ctx) }
                <button
                  onclick={close_cb}
                  class="modal-close is-large"
                  aria-label="close"
                 ></button>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.cameras_loaded {
            // cameras already loaded
            return;
        }

        let window = web_sys::window().unwrap();
        // prepare js instances
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().unwrap();
        let devices = media_devices.enumerate_devices();

        if self.active {
            // Obtain media info in the call
            ctx.link().send_future(async move {
                let device_infos = JsFuture::from(devices.unwrap()).await.unwrap();

                let cameras: Vec<Camera> = js_sys::try_iter(&device_infos)
                    .unwrap()
                    .unwrap()
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.dyn_into::<MediaDeviceInfo>().ok())
                    .filter(|e| e.kind() == MediaDeviceKind::Videoinput)
                    .map(|e| Camera {
                        device_id: e.device_id(),
                        label: e.label(),
                        icon: "fas fa-video".to_string(),
                        disabled: false,
                    })
                    .collect();

                Msg::CamerasLoaded(cameras)
            });
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        true
    }
}

impl QRScanner {
    fn video_view(&self, ctx: &Context<Self>) -> Html {
        if self.active {
            let text = translations::get_message_global("no_camera", &ctx.props().lang, None);
            let no_camera_html = if self.camera_available {
                html! {}
            } else {
                html! {
                    <div class="content is-flex is-justify-content-center">
                        <div class="notification is-danger w-50"><p>{{ text }}</p></div>
                    </div>
                }
            };

            html! {
                <div class="section">
                    <video
                      width="auto"
                      height="auto"
                      id="video"
                      ref={self.video_ref.clone()}
                      style={
                        if self.camera_available {"max-width:80%;max-height:80%"} else {"display:none"}
                      }
                      poster="images/qrcode.svg"
                    ></video>
                    <canvas
                      width=640
                      height=480
                      id="canvas"
                      style="display: none;"
                      ref={self.canvas_ref.clone()}>
                    </canvas>
                    { no_camera_html }
                </div>
            }
        } else {
            html! {}
        }
    }

    fn view_cameras(&self, ctx: &Context<Self>) -> Html {
        let render_camera = |camera: &Camera| {
            let active = Some(&camera.device_id) == self.current_device_id.as_ref();
            let class = if active {
                classes!("is-active")
            } else {
                classes!("")
            };
            let device_id = camera.device_id.clone();
            let onclick = ctx.link().callback(move |_| {
                if active {
                    Msg::SwitchCamera(None)
                } else {
                    Msg::SwitchCamera(Some(device_id.clone()))
                }
            });
            html! {
                <li class = { class }>
                  <a { onclick } disabled={ camera.disabled }>
                    <span class="icon is-small"><i class={ camera.icon.to_string() } aria-hidden="true"></i></span>
                    <span>{ camera.label.to_string() }</span>
                  </a>
                </li>
            }
        };
        html! {
            <div class="tabs is-centered is-toggle">
                <ul class="ml-0">
                    { for self.cameras.iter().map(render_camera) }
                </ul>
            </div>
        }
    }

    fn view_text_input(&self, ctx: &Context<Self>) -> Html {
        let toggle_cb = ctx.link().callback(|_| Msg::ToggleTextInput);
        let input_ref = self.input_ref.clone();
        let trigger_cb = ctx.link().callback(move |_: MouseEvent| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            Msg::TextInput(input.value())
        });
        if self.show_text_input {
            html! {
              <div class="field has-addons is-hidden-touch">
                <p class="control">
                  <a class="button is-light" onclick={toggle_cb}>
                    <span class="icon is-small"><i class="fas fa-chevron-right" aria-hidden="true"></i></span>
                  </a>
                </p>
                <p class="control">
                  <input
                    class="input"
                    type="text"
                    ref={self.input_ref.clone()}
                  />
                </p>
                <p class="control">
                  <a class="button is-info" onclick={trigger_cb}>
                    <span class="icon is-small"><i class="fas fa-keyboard" aria-hidden="true"></i></span>
                  </a>
                </p>
              </div>
            }
        } else {
            html! {
              <div class="field has-addons is-hidden-touch">
                <p class="control">
                  <a class="button is-info" onclick={toggle_cb}>
                    <span class="icon is-small"><i class="fas fa-keyboard" aria-hidden="true"></i></span>
                  </a>
                </p>
              </div>
            }
        }
    }

    fn prepare_interval(&mut self, ctx: &Context<Self>) {
        // Terminate existing
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }

        let link = ctx.link().clone();

        let video = self.video_ref.cast::<HtmlVideoElement>().unwrap();
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let context_js: JsValue = canvas.get_context("2d").unwrap().unwrap().into();
        let context: CanvasRenderingContext2d = context_js.into();
        let (width, height) = if video.video_width() > video.video_height() {
            (640.0, 480.0)
        } else {
            (480.0, 640.0)
        };

        // Plan new interval and start rendering
        let interval = gloo::timers::callback::Interval::new(100, move || {
            context
                .draw_image_with_html_video_element_and_dw_and_dh(&video, 0.0, 0.0, width, height)
                .unwrap();
            if let Ok(data) =
                context.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
            {
                // Try to extract QR code from the picture
                link.send_message(Msg::Picture(data));
            }
        });

        self.interval = Some(interval);
    }

    fn turn_off_camera(&self) {
        if let Some(video) = self.video_ref.cast::<HtmlVideoElement>() {
            if let Some(stream) = video.src_object() {
                let tracks = stream.get_tracks();
                tracks.iter().for_each(|e| {
                    e.dyn_into::<MediaStreamTrack>()
                        .map(|t| t.stop())
                        .unwrap_or(())
                });
                // Stop all streams
                video.set_src_object(None);
                // Reset video
                video.load();
            }
        }
    }
}
