use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::translations::get_message_global;

#[component]
pub fn Title<G: Html>(ctx: ScopeRef) -> View<G> {
    let subtitle = get_message_global("subtitle", "cs", None);
    view! { ctx,
        section(
            class="hero is-clickable",
            on:click=|_| { navigate("/") },
        ) {
            div(class="hero-body") {
                div(class="container has-text-centered") {
                    p(class="title") {"Pabitell"}
                    p(class="subtitle") { (subtitle) }
                }
            }
        }
    }
}
