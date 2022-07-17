use super::characters;
use fluent::{FluentArgs, FluentValue};
use std::rc::Rc;
use uuid::Uuid;
use yew::{html, prelude::*};

use crate::translations::get_message_global;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub character: Rc<characters::Character>,
    pub world_id: Uuid,
    pub show_qr_cb: Callback<Rc<Vec<u8>>>,
    pub lang: String,
}

pub struct ActionJoin {}

pub enum Msg {
    ShowQRCode,
}

impl Component for ActionJoin {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowQRCode => {
                log::info!("QR active");

                let character = ctx.props().character.clone();
                let character_code: Option<String> = character.code.as_ref().clone();
                let data = characters::CharacterQRJson::new(character_code, ctx.props().world_id);
                let data = Rc::new(serde_json::to_vec(&data).unwrap());
                ctx.props().show_qr_cb.emit(data);
            }
        }
        false
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ShowQRCode);
        let character = ctx.props().character.clone();

        let mut f_args = FluentArgs::new();
        f_args.set(
            "characterName",
            FluentValue::from(character.short.to_string()),
        );

        let help_text = get_message_global("will_join_the_game", &ctx.props().lang, Some(f_args));

        html! {
            <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-48x48">
                                <img src={"images/sign-in-alt.svg"} />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-4">{character.short.clone()}</p>
                            <p class="subtitle is-6"></p>
                        </div>
                    </div>
                </div>
                <div class="card-image has-text-centered">
                    <figure class="image is-clickable is-square w-75 is-inline-block" {onclick} >
                        <img class="box" src={character.character_url.to_string()}/>
                    </figure>
                    <div class="content">{help_text}</div>
                </div>
            </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }
}
