use pabitell_lib::{Character, Description, World, WorldBuilder};
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

use super::characters;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub available_characters: Rc<Vec<Rc<characters::Character>>>,
    pub set_character: Callback<Rc<Option<String>>>,
    pub selected_character: Rc<Option<String>>,
    pub fixed: bool,
}

pub struct CharacterSwitch {}

pub enum Msg {
    UpdateSelectedCharacter(Rc<characters::Character>),
    Void,
}

impl Component for CharacterSwitch {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSelectedCharacter(selected_character) => {
                ctx.props()
                    .set_character
                    .emit(selected_character.code.clone());
            }
            Msg::Void => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let characters = ctx.props().available_characters.clone();
        let selected_character_code = ctx.props().selected_character.clone();
        let render_character = move |character: &Rc<characters::Character>| {
            let cloned_character: Rc<characters::Character> = character.clone();
            let onclick =
                link.callback(move |_| Msg::UpdateSelectedCharacter(cloned_character.clone()));
            if ctx.props().fixed {
                html! {
                    <li class={ if selected_character_code == character.code { "is-active" } else { "" } }>
                      <a class="button" disabled={selected_character_code != character.code}>
                        <span class="icon is-small">
                            <i class={ character.icon.to_string() }></i>
                        </span>
                        <span>{ character.short.clone() }</span>
                      </a>
                    </li>
                }
            } else {
                html! {
                    <li class={ if selected_character_code == character.code { "is-active" } else { "" } }>
                      <a {onclick} class="button">
                        <span class="icon is-small">
                            <i class={ character.icon.to_string() }></i>
                        </span>
                        <span>{ character.short.clone() }</span>
                      </a>
                    </li>
                }
            }
        };

        html! {
            <>
                <div class="tabs is-toggle is-fullwidth is-small is-hidden-desktop">
                    <ul>
                        { for characters.iter().map(render_character.clone()) }
                    </ul>
                </div>
                <div class="tabs is-toggle is-large is-centered is-hidden-touch">
                    <ul>
                        { for characters.iter().map(render_character) }
                    </ul>
                </div>
            </>
        }
    }
}
