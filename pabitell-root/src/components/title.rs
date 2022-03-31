use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::translations::get_message_global;

#[derive(Prop)]
pub struct TitleProps<'a> {
    selected_language: &'a Signal<String>,
}

#[component]
pub fn Title<'a, G: Html>(ctx: ScopeRef<'a>, props: TitleProps<'a>) -> View<G> {
    view! { ctx,
        section(
            class="hero is-clickable",
            on:click=|_| { navigate("/") },
        ) {
            div(class="hero-body") {
                div(class="container has-text-centered") {
                    p(class="title") {"Pabitell"}
                    p(class="subtitle") {
                        (
                            get_message_global(
                               "subtitle",
                                props.selected_language.get().as_ref(),
                                None
                            )
                        )
                    }
                }
            }
        }
    }
}
