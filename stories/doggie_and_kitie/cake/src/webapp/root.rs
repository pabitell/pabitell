use pabitell_lib::{
    webapp::{app::App, items::Item, print::PrintItem},
    Narrator, World,
};
use std::rc::Rc;
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = "doggie_and_kitie_cake";
        let make_characters: Option<Box<dyn Fn(&dyn World) -> Rc<Vec<Rc<_>>>>> =
            Some(Box::new(make_characters));
        let make_narrator: Option<Box<dyn Fn() -> Box<dyn Narrator>>> =
            Some(Box::new(make_narrator));
        let make_world: Option<Box<dyn Fn(&str) -> Box<dyn World>>> = Some(Box::new(make_world));
        let make_print_items: Option<Box<dyn Fn(Box<dyn World>) -> Vec<PrintItem>>> =
            Some(Box::new(make_print_items));
        let make_owned_items: Option<
            Box<dyn Fn(&dyn World, &Option<String>) -> Rc<Vec<Rc<Item>>>>,
        > = Some(Box::new(make_owned_items));
        let make_languages: Option<Box<dyn Fn() -> Rc<Vec<String>>>> =
            Some(Box::new(make_languages));
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
