use pabitell_lib::webapp::app::{
    App, MakeCharacters, MakeLanguages, MakeNarrator, MakeOwnedItems, MakePrintItems, MakeWorld,
};
use yew::prelude::*;

use super::{
    characters::make_characters, items::make_owned_items, narrator::make_narrator,
    print_items::make_print_items, world::make_world,
};
use crate::translations::make_languages;

pub struct Root {}

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let name = "doggie_and_kitie_doll";
        let make_characters: MakeCharacters = Some(Box::new(make_characters));
        let make_narrator: MakeNarrator = Some(Box::new(make_narrator));
        let make_world: MakeWorld = Some(Box::new(make_world));
        let make_print_items: MakePrintItems = Some(Box::new(make_print_items));
        let make_owned_items: MakeOwnedItems = Some(Box::new(make_owned_items));
        let make_languages: MakeLanguages = Some(Box::new(make_languages));
        html! {
            <App
              {make_characters}
              {make_narrator}
              {make_print_items}
              {make_owned_items}
              {make_world}
              {make_languages}
              {name}
            >
            </App>
        }
    }
}
