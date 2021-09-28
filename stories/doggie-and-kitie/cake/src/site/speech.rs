use std::{
    cell::RefCell,
    collections::VecDeque,
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
        Element, EventTarget, HtmlCanvasElement, HtmlMediaElement, HtmlSelectElement,
        HtmlVideoElement, MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream,
        MediaStreamConstraints, MediaStreamTrack, SpeechSynthesis, SpeechSynthesisUtterance,
        SpeechSynthesisVoice,
    },
    Event,
};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub start_text: String,
    pub lang: String,
    pub shared_scope: Rc<RefCell<Option<html::Scope<Speech>>>>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, rhs: &Self) -> bool {
        rhs.lang == self.lang
    }
}

pub struct Speech {
    voice: Option<SpeechSynthesisVoice>,
    voices: Vec<SpeechSynthesisVoice>,
    queue: VecDeque<SpeechSynthesisUtterance>,
    playing: bool,
    end: Closure<dyn Fn()>,
}

pub enum Msg {
    Play(String),
    End,
    SetVoice(Option<usize>),
}

impl Component for Speech {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Play(text) => {
                let text = text.replace("\n", " ").replace(".", ".\n"); // converts string for better tts
                let synth = web_sys::window().unwrap().speech_synthesis().unwrap();
                // Already speaking
                if let Some(voice) = self.voice.as_ref() {
                    let mut ut = SpeechSynthesisUtterance::new().unwrap();
                    ut.set_text(&text);
                    ut.set_pitch(1.0);
                    ut.set_rate(1.0);
                    ut.set_voice(Some(voice));

                    if self.playing {
                        self.queue.push_back(ut);
                    } else {
                        ut.set_onend(Some(self.end.as_ref().unchecked_ref()));
                        self.playing = true;
                        synth.cancel();
                        synth.speak(&ut);
                    }
                }
                false
            }
            Msg::SetVoice(idx) => {
                if let Some(idx) = idx {
                    self.voice = Some(self.voices[idx].clone());
                } else {
                    self.voice = None;
                }
                ctx.link()
                    .send_message(Msg::Play(ctx.props().start_text.clone()));
                true
            }
            Msg::End => {
                if let Some(ut) = self.queue.pop_front() {
                    let synth = web_sys::window().unwrap().speech_synthesis().unwrap();
                    ut.set_onend(Some(self.end.as_ref().unchecked_ref()));
                    synth.cancel();
                    synth.speak(&ut);
                } else {
                    self.playing = false;
                }
                false
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        let window = web_sys::window().unwrap();

        let voices = window.speech_synthesis().unwrap().get_voices();
        let voices = voices
            .iter()
            .filter_map(|e| e.dyn_into::<SpeechSynthesisVoice>().ok())
            .filter(|e| e.lang().starts_with(&ctx.props().lang))
            .collect();

        let link = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move || {
            link.send_future(async move { Msg::End });
        }) as Box<dyn Fn()>);

        Self {
            voice: None,
            voices,
            queue: VecDeque::new(),
            playing: false,
            end: closure,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            input.map(|input| {
                if let Ok(idx) = input.value().parse::<usize>() {
                    Msg::SetVoice(Some(idx))
                } else {
                    Msg::SetVoice(None)
                }
            })
        });
        let render_voice = |(idx, voice): (usize, &SpeechSynthesisVoice)| {
            html! {
                <option value={idx.to_string()}>{ voice.name() }</option>
            }
        };

        html! {
            <div class="control has-icons-left">
              <div class="select">
                <select {onchange}>
                  <option value={""} selected={true}>{"---"}</option>
                  { for self.voices.iter().enumerate().map(render_voice) }
                </select>
              </div>
              <div class="icon is-small is-left">
                <i class="fas fa-bullhorn"></i>
              </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());
        true
    }
}