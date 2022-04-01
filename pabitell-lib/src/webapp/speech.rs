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
use web_sys::{
    Blob, CanvasRenderingContext2d, ConstrainBooleanParameters, DisplayMediaStreamConstraints,
    Element, EventTarget, HtmlCanvasElement, HtmlMediaElement, HtmlSelectElement, HtmlVideoElement,
    MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStream, MediaStreamConstraints,
    MediaStreamTrack, SpeechSynthesis, SpeechSynthesisUtterance, SpeechSynthesisVoice,
};
use yew::{function_component, html, prelude::*, Event};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub start_text: String,
    pub lang: String,
    pub shared_scope: Rc<RefCell<Option<html::Scope<Speech>>>>,
    pub world_name: String,
}

fn storage_world_prefix(world_name: &str, key: &str) -> String {
    format!("{}:{}", world_name, key)
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
    retry_texts: Vec<String>,
    playing: bool,
    end: Closure<dyn Fn()>,
    onvoiceschanged: Closure<dyn Fn()>,
    default_voice_key: String,
    current_lang: String,
}

pub enum Msg {
    Play(String),
    End,
    SetVoice(Option<usize>),
    GetVoiceList,
}

impl Component for Speech {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Play(text) => {
                let text = text
                    .replace("\n", " ")
                    .replace(".", ".\n")
                    .replace("\"", "")
                    .replace("„", "")
                    .replace("“", ""); // converts string for better tts
                let synth = web_sys::window().unwrap().speech_synthesis().unwrap();
                if let Some(voice) = self.voice.as_ref() {
                    let mut ut = SpeechSynthesisUtterance::new().unwrap();
                    ut.set_text(&text);
                    ut.set_pitch(1.0);
                    ut.set_rate(1.0);
                    ut.set_voice(Some(voice));

                    // Already speaking
                    if self.playing {
                        self.queue.push_back(ut);
                    } else {
                        ut.set_onend(Some(self.end.as_ref().unchecked_ref()));
                        self.playing = true;
                        synth.cancel();
                        synth.speak(&ut);
                    }
                } else {
                    // Some voice was set in the past
                    // but speech was not initialized yet
                    if (storage::LocalStorage::get(&self.default_voice_key) as Result<String, _>)
                        .is_ok()
                    {
                        self.retry_texts.push(text);
                    }
                }
                false
            }
            Msg::SetVoice(idx) => {
                if let Some(idx) = idx {
                    storage::LocalStorage::set(&self.default_voice_key, &self.voices[idx].name())
                        .unwrap();
                    self.voice = Some(self.voices[idx].clone());
                } else {
                    self.voice = None;
                    storage::LocalStorage::delete(&self.default_voice_key);
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
            Msg::GetVoiceList => {
                let voices = web_sys::window()
                    .unwrap()
                    .speech_synthesis()
                    .unwrap()
                    .get_voices();
                self.voices = voices
                    .iter()
                    .filter_map(|e| e.dyn_into::<SpeechSynthesisVoice>().ok())
                    .filter(|e| e.lang().starts_with(&self.current_lang))
                    .collect();

                if let Ok(selected_voice) =
                    storage::LocalStorage::get(&self.default_voice_key) as Result<String, _>
                {
                    for (idx, voice) in self.voices.iter().enumerate() {
                        if voice.name() == selected_voice {
                            self.voice = Some(self.voices[idx].clone());

                            let link = ctx.link();
                            // Play the texts which should be retried
                            self.retry_texts
                                .drain(..)
                                .for_each(|e| link.send_message(Msg::Play(e)));
                        }
                    }
                }
                true
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());

        let link = ctx.link().clone();

        // Update voices
        link.send_future(async move { Msg::GetVoiceList });

        // Set on voice changed handler
        let synth = web_sys::window().unwrap().speech_synthesis().unwrap();
        let cloned_link = link.clone();
        let onvoiceschanged = Closure::wrap(Box::new(move || {
            log::info!("Synth voices updated");
            cloned_link.send_future(async move { Msg::GetVoiceList })
        }) as Box<dyn Fn()>);
        synth.set_onvoiceschanged(Some(onvoiceschanged.as_ref().unchecked_ref()));

        let end = Closure::wrap(Box::new(move || {
            link.send_future(async move { Msg::End });
        }) as Box<dyn Fn()>);

        Self {
            voice: None,
            voices: vec![],
            queue: VecDeque::new(),
            retry_texts: vec![],
            playing: false,
            end,
            current_lang: ctx.props().lang.clone(),
            onvoiceschanged,
            default_voice_key: storage_world_prefix(&ctx.props().world_name, "speech"),
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

        let default_voice: Option<String> =
            storage::LocalStorage::get(&self.default_voice_key).ok();
        let render_voice = |(idx, voice): (usize, &SpeechSynthesisVoice)| {
            if let Some(default_voice) = default_voice.clone() {
                if default_voice == voice.name() {
                    return html! {
                        <option value={idx.to_string()} selected={true}>{ voice.name() }</option>
                    };
                }
            }
            html! {
                <option value={idx.to_string()}>{ voice.name() }</option>
            }
        };
        let render_empty_option = || {
            if default_voice.is_some() {
                html! {
                  <option value={""}>{"---"}</option>
                }
            } else {
                html! {
                  <option value={""} selected={true}>{"---"}</option>
                }
            }
        };

        html! {
            <div class="control has-icons-left">
                <div class="select">
                    <select {onchange}>
                        { render_empty_option() }
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
        if *ctx.props().lang != self.current_lang {
            self.current_lang = ctx.props().lang.clone();
            ctx.link().send_message(Msg::GetVoiceList);
        }
        true
    }
}
