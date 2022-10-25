use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use gloo::storage::{self, Storage};
use js_sys::Array;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use web_sys::{
    EventTarget, HtmlSelectElement, SpeechSynthesis, SpeechSynthesisUtterance, SpeechSynthesisVoice,
};
use yew::{html, prelude::*, Event};

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
    #[allow(dead_code)]
    onvoiceschanged: Option<Closure<dyn Fn()>>,
    default_voice_key: String,
    current_lang: String,
}

pub enum Msg {
    Play(String),
    End,
    SetVoice(Option<usize>),
    GetVoiceList,
    UpdateVoices(Array),
    UpdateOnVoiceChanged(Option<Closure<dyn Fn()>>),
}

impl Component for Speech {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Play(text) => {
                let text = text
                    .replace('\n', " ")
                    .replace('.', ".\n")
                    .replace('\"', "")
                    .replace('„', "")
                    .replace('“', ""); // converts string for better tts
                let synth: SpeechSynthesis = web_sys::window().unwrap().speech_synthesis().unwrap();
                if let Some(voice) = self.voice.as_ref() {
                    let ut = SpeechSynthesisUtterance::new().unwrap();
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
                if self.onvoiceschanged.is_some() {
                    // Updating voices under way
                    return false;
                }

                let cloned_link = ctx.link().clone();
                // Load voices in async mode
                // Create js promise for that
                let promise = future_to_promise(async {
                    log::info!("Obtaining list voices directly");
                    let voices = web_sys::window()
                        .unwrap()
                        .speech_synthesis()
                        .unwrap()
                        .get_voices();

                    if voices.length() == 0 {
                        // Some browsers needs to be async
                        let link = cloned_link.clone();
                        let onvoiceschanged = Closure::wrap(Box::new(move || {
                            log::info!("Obtaining voices in onvoiceschanged handler");
                            let voices = web_sys::window()
                                .unwrap()
                                .speech_synthesis()
                                .unwrap()
                                .get_voices();
                            cloned_link.send_future(async move { Msg::UpdateVoices(voices) })
                        })
                            as Box<dyn Fn()>);

                        link.send_message(Msg::UpdateOnVoiceChanged(Some(onvoiceschanged)));

                        // Valid list was not fetched during first attempt
                        Err(JsValue::NULL)
                    } else {
                        cloned_link.send_message(Msg::UpdateVoices(voices.clone()));
                        Ok(voices.into())
                    }
                });

                // Wait for promise to finish and update voices afterwards
                let link = ctx.link().clone();
                spawn_local(async move {
                    let future: JsFuture = promise.into();
                    if let Ok(voices) = future.await {
                        link.send_message(Msg::UpdateVoices(voices.into()));
                    }
                });

                false
            }
            Msg::UpdateVoices(voices) => {
                let link = ctx.link();
                // needed to use unchecked_into because in some browsers
                // dyn_into doesn't work well here
                self.voices = voices
                    .iter()
                    .map(|e| e.unchecked_into::<SpeechSynthesisVoice>())
                    .filter(|e| e.lang().starts_with(&self.current_lang))
                    .collect();

                log::debug!(
                    "Voices obtained count (total/filtered) - ({:?}/{:?})",
                    &voices.length(),
                    self.voices.len()
                );

                // Clean the closure after success
                link.send_message(Msg::UpdateOnVoiceChanged(None));

                if let Ok(selected_voice) =
                    storage::LocalStorage::get(&self.default_voice_key) as Result<String, _>
                {
                    for (idx, voice) in self.voices.iter().enumerate() {
                        if voice.name() == selected_voice {
                            self.voice = Some(self.voices[idx].clone());

                            // Play the texts which should be retried
                            self.retry_texts
                                .drain(..)
                                .for_each(|e| link.send_message(Msg::Play(e)));
                        }
                    }
                }
                true
            }
            Msg::UpdateOnVoiceChanged(onvoiceschanged_opt) => {
                log::debug!("Setting onvoiceschanged {:?}", onvoiceschanged_opt);
                let synth: SpeechSynthesis = web_sys::window().unwrap().speech_synthesis().unwrap();
                // Set callback
                synth.set_onvoiceschanged(
                    onvoiceschanged_opt
                        .as_ref()
                        .map(|e| e.as_ref().unchecked_ref()),
                );
                // Need to store closure to avoid its destruction
                self.onvoiceschanged = onvoiceschanged_opt;
                false
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().shared_scope.borrow_mut() = Some(ctx.link().clone());

        let link = ctx.link().clone();

        // Update voices
        link.send_future(async move { Msg::GetVoiceList });

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
            default_voice_key: storage_world_prefix(&ctx.props().world_name, "speech"),
            onvoiceschanged: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange = link.batch_callback(|e: Event| {
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

        let refresh_disabled = self.onvoiceschanged.is_some();
        let refresh_classes = if refresh_disabled {
            classes!("fas", "fa-arrows-rotate", "rotate")
        } else {
            classes!("fas", "fa-arrows-rotate")
        };

        let refresh_cb = link.callback(|_| Msg::GetVoiceList);

        html! {
            <>
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
                <button class="button is-info is-outlined" disabled={refresh_disabled} onclick={refresh_cb}>
                    <i class={refresh_classes}></i>
                </button>
            </>
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
