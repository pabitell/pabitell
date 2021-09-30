use std::{
    cell::RefCell,
    convert::TryInto,
    rc::Rc,
    sync::{Arc, Mutex},
};

use gloo::{
    dialogs,
    storage::{self, Storage},
    timers,
};
use js_sys::{ArrayBuffer, Function, Uint8Array};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::{
    function_component, html,
    prelude::*,
    web_sys::{
        Blob, CanvasRenderingContext2d, ConstrainBooleanParameters, DisplayMediaStreamConstraints,
        Element, EventTarget, HtmlCanvasElement, HtmlMediaElement, HtmlVideoElement,
        MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream, MediaStreamConstraints,
        MediaStreamTrack,
    },
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub qr_found: Callback<String>,
    pub shared_scope: Rc<RefCell<Option<html::Scope<QRScanner>>>>,
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
    interval: Option<timers::callback::Interval>,
}

pub enum Msg {
    Active(bool),
    CamerasLoaded(Vec<Camera>),
    Picture(Vec<u8>),
    SwitchCamera(Option<String>),
    CameraSwitched,
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
                    }
                } else {
                    // restore previous view on re-open
                    if let Some(device_id) = self.current_device_id.as_ref() {
                        ctx.link()
                            .send_message(Msg::SwitchCamera(Some(device_id.clone())));
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
                // Try to found QR code in the picture
                let image = image::load_from_memory(&data).unwrap();
                let mut builder = bardecoder::default_builder();
                builder.prepare(Box::new(bardecoder::prepare::BlockedMean::new(8, 57)));
                let decoder = builder.build();
                let results = decoder.decode(&image);
                let mut results: Vec<_> = results.into_iter().filter_map(|e| e.ok()).collect();
                if results.len() > 0 {
                    // use last found QR
                    ctx.props().qr_found.emit(results.pop().unwrap());
                    if let Some(interval) = self.interval.take() {
                        interval.cancel();
                    }
                    self.active = false;
                    true
                } else {
                    false
                }
            }
            Msg::SwitchCamera(device_id) => {
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

                let media = self.video_ref.cast::<HtmlMediaElement>().unwrap();

                // Turn camera off
                if let Some(stream) = media.src_object() {
                    let tracks = stream.get_tracks();
                    tracks.iter().for_each(|e| {
                        e.dyn_into::<MediaStreamTrack>()
                            .map(|t| t.stop())
                            .unwrap_or(())
                    });
                    // Stop all streams
                    media.set_src_object(None);
                    // Reset video
                    media.load();
                }
                let video = self.video_ref.cast::<HtmlVideoElement>().unwrap();
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

                if let Some(device_id) = device_id.clone() {
                    // Turn camera on
                    ctx.link().send_future(async move {
                        let mut constraints = MediaStreamConstraints::new();
                        constraints.video(
                            &JsValue::from_serde(
                                &serde_json::json!({"deviceId": {"exact": device_id}}),
                            )
                            .unwrap(),
                        );
                        let stream_fut =
                            media_devices.get_user_media_with_constraints(&constraints);

                        let stream_js = JsFuture::from(stream_fut.unwrap()).await.unwrap();
                        let media_stream = MediaStream::from(stream_js);

                        media.set_src_object(Some(&media_stream));
                        JsFuture::from(media.play().unwrap()).await.unwrap();

                        canvas.set_width(video.video_width());
                        canvas.set_height(video.video_height());

                        Msg::CameraSwitched
                    });
                }
                self.current_device_id = device_id.clone();
                true
            }

            Msg::CameraSwitched => {
                self.prepare_interval(ctx);
                false
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            active: false,
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            interval: None,
            cameras_loaded: false,
            cameras: vec![Camera {
                icon: "fas fa-spinner fa-spin".into(),
                label: "".into(),
                device_id: "".into(),
                disabled: true,
            }],
            current_device_id: None,
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
                { self.view_cameras(ctx) }
                { self.video_view() }
                </div>
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
    fn video_view(&self) -> Html {
        if self.active {
            html! {
                <div class="section">
                    <video
                      width="auto"
                      height="auto"
                      id="video"
                      ref={self.video_ref.clone()}
                      style="max-width:80%;max-height:80%"
                      poster="images/qrcode.svg"
                    ></video>
                    <canvas
                      width=640
                      height=480
                      id="canvas"
                      style="display: none;"
                      ref={self.canvas_ref.clone()}>
                    </canvas>
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
                <ul>
                    { for self.cameras.iter().map(render_camera) }
                </ul>
            </div>
        }
    }

    fn prepare_interval(&mut self, ctx: &Context<Self>) {
        // Terminate existing
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }

        let link = ctx.link().clone();
        // Plan new interval and start rendering
        let closure = Closure::wrap(Box::new(move |blob: Blob| {
            link.send_future(async move {
                // Reading entire file to an array and export it
                let array_buffer: ArrayBuffer =
                    JsFuture::from(blob.array_buffer()).await.unwrap().into();

                // TODO implement Read over wrapped array
                let array = Uint8Array::new(&array_buffer);

                let new_len: usize = array_buffer.byte_length() as usize;
                let mut data: Vec<u8> = vec![0; new_len];
                array.copy_to(&mut data);

                Msg::Picture(data)
            })
        }) as Box<dyn Fn(Blob)>);

        let video = self.video_ref.cast::<HtmlVideoElement>().unwrap();
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let context_js: JsValue = canvas.get_context("2d").unwrap().unwrap().into();
        let context: CanvasRenderingContext2d = context_js.into();
        let (width, height) = if video.video_width() > video.video_height() {
            (640.0, 480.0)
        } else {
            (480.0, 640.0)
        };
        let interval = gloo::timers::callback::Interval::new(100, move || {
            context
                .draw_image_with_html_video_element_and_dw_and_dh(&video, 0.0, 0.0, width, height)
                .unwrap();
            canvas.to_blob(closure.as_ref().unchecked_ref()).unwrap();
        });

        self.interval = Some(interval);
    }
}
