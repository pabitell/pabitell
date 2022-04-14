use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(ctx: Scope) -> View<G> {
    view! { ctx,
        footer(class="footer") {
            div(class="content has-text-centered") {
                a(href="https://github.com/shenek/pabitell/", rel="external"){
                    "Pabitell"
                }
            }
        }
    }
}
