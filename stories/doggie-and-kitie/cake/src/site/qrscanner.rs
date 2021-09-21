use std::{
    cell::RefCell,
    convert::TryInto,
    rc::Rc,
    sync::{Arc, Mutex},
};

use gloo::{dialogs, timers};
use js_sys::{ArrayBuffer, Function, Uint8Array};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::{
    function_component, html,
    prelude::*,
    web_sys::{
        Blob, CanvasRenderingContext2d, Element, EventTarget, HtmlCanvasElement, HtmlMediaElement,
        HtmlVideoElement, MediaDevices, MediaStream, MediaStreamConstraints,
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

pub struct QRScanner {
    active: bool,
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    interval: Option<timers::callback::Interval>,
}

pub enum Msg {
    Active(bool),
    DeviceDetailsFound(timers::callback::Interval),
    Picture(Vec<u8>),
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
                }
                true
            }
            Msg::DeviceDetailsFound(interval) => {
                // Start timer
                if let Some(interval) = self.interval.take() {
                    interval.cancel();
                }
                self.interval = Some(interval);

                false
            }
            Msg::Picture(data) => {
                // Try to found QR code in the picture
                let image = image::load_from_memory(&data).unwrap();
                let decoder = bardecoder::default_decoder();
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
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            active: false,
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            interval: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_cb = ctx.link().callback(|_| Msg::Active(false));
        let classes = if self.active {
            classes!("modal", "is-active")
        } else {
            classes!("modal")
        };

        let content = if self.active {
            html! {
                <>
                    <video width="640" height="480" id="video" ref={self.video_ref.clone()} style="max-width:80%;max-height:80%"></video>
                    <canvas width="640px" height="480px" id="canvas" style="display: none;" ref={self.canvas_ref.clone()}></canvas>
                </>
            }
        } else {
            html! {}
        };

        html! {
            <div class={classes}>
                <div class="modal-background"></div>
                <div class="modal-content has-text-centered">
                { content }
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
        let window = web_sys::window().unwrap();
        // prepare js instances
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_serde(&serde_json::json!({"video": true})).unwrap());
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().unwrap();
        let devices = media_devices.get_user_media_with_constraints(&constraints);

        if let Some(media) = self.video_ref.cast::<HtmlMediaElement>() {
            // Get html elemtns
            let video = self.video_ref.cast::<HtmlVideoElement>().unwrap();
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            // Get canvas 2d context
            let context_js: JsValue = canvas.get_context("2d").unwrap().unwrap().into();
            let context: CanvasRenderingContext2d = context_js.into();

            let inner_link = ctx.link().clone();

            // Obtain media info in the call
            ctx.link().send_future(async move {
                let stream_js = JsFuture::from(devices.unwrap()).await.unwrap();
                let media_stream = MediaStream::from(stream_js);
                media.set_src_object(Some(&media_stream));
                JsFuture::from(media.play().unwrap()).await.unwrap();

                let closure = Closure::wrap(Box::new(move |blob: Blob| {
                    inner_link.send_future(async move {
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

                let interval = gloo::timers::callback::Interval::new(1_000, move || {
                    context
                        .draw_image_with_html_video_element(&video, 0.0, 0.0)
                        .unwrap();
                    canvas.to_blob(closure.as_ref().unchecked_ref()).unwrap();
                });

                Msg::DeviceDetailsFound(interval)
            });
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        true
    }
}
