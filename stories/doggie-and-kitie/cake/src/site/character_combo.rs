use pabitell_lib::{Character, Description, World, WorldBuilder};
use std::rc::Rc;
use yew::prelude::*;

use crate::{translations::get_message, world::CakeWorld};

use super::characters;

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {
    pub available_characters: Vec<Rc<characters::Character>>,
    pub set_character: Callback<Rc<Option<String>>>,
}

pub struct CharacterCombo {
    pub selected_character: Rc<characters::Character>,
}

pub enum Msg {
    UpdateSelectedCharacter(Rc<characters::Character>),
}

impl Component for CharacterCombo {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected_character: ctx.props().available_characters[0].clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSelectedCharacter(selected_character) => {
                self.selected_character = selected_character.clone();
                ctx.props()
                    .set_character
                    .emit(selected_character.code.clone());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let render_icon = |icon: Rc<String>, name: Rc<String>| {
            if !icon.is_empty() {
                html! {
                    <span class="icon-text">
                        <span class="icon">
                        <i class={ icon.to_string() }></i>
                        </span>
                        <span>{ name.to_string() }</span>
                    </span>
                }
            } else {
                html! {<span>{name}</span>}
            }
        };

        let render_character = move |item: Rc<characters::Character>| {
            if self.selected_character.code == item.code {
                html! {
                    <a class="navbar-item is-active">{ item.short.clone() }</a>
                }
            } else {
                let cloned = item.clone();
                let cb = link.callback(move |_| Msg::UpdateSelectedCharacter(cloned.clone()));
                html! {
                    <a class="navbar-item" onclick={ cb }>{ item.short.clone() }</a>
                }
            }
        };

        let inner = render_icon(
            self.selected_character.icon.clone(),
            self.selected_character.short.clone(),
        );

        html! {
          <div class="navbar-item has-dropdown is-hoverable">
            <a class="navbar-link">{ inner }</a>

            <div class="navbar-dropdown">
            { for ctx.props().available_characters.clone().into_iter().map(render_character) }
            </div>
          </div>
        }
    }
}
