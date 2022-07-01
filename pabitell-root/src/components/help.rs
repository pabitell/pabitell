use sycamore::prelude::*;

use crate::translations::get_message_global;

#[derive(Prop)]
pub struct HelpProps<'a> {
    lang: &'a Signal<String>,
}

#[component]
pub fn Help<'a, G: Html>(ctx: Scope<'a>, props: HelpProps<'a>) -> View<G> {
    let show_help = false;
    let help_signal = create_signal(ctx, show_help);

    view! { ctx,
        div(class="control pr-2") {
            button(
                class="button",
                on:click=|_| { help_signal.set(if *help_signal.get() { false } else { true} ) },
            ) { strong{"?"} }
        }
        div(
            class=if *help_signal.get() { "modal is-active" } else { "modal" }
        ) {
            div(class="modal-background")
            div(class="modal-card") {
                header(class="modal-card-head") {
                    p(class="modal-card-title mb-0") {
                        (
                            get_message_global("help_title", props.lang.get().as_ref(), None)
                        )
                    }
                    button(
                        class="delete",
                        aria-label="close",
                        on:click=|_| { help_signal.set(false) },
                    )
                }
                section(class="modal-card-body") {
                    p {
                        (
                            get_message_global("help_text", props.lang.get().as_ref(), None)
                        )
                    }
                    ul {
                        li {
                            span(class="icon has-text-success") { i(class="fas fa-plus-circle") }
                            ( get_message_global("help_text_create", props.lang.get().as_ref(), None) )
                        }
                        li {
                            span(class="icon has-text-info") { i(class="fas fa-sign-in-alt") }
                            ( get_message_global("help_text_join", props.lang.get().as_ref(), None) )
                        }
                        li {
                            span(class="icon has-text-dark") { i(class="fas fa-print") }
                            ( get_message_global("help_text_print", props.lang.get().as_ref(), None) )
                        }
                    }
                    p {
                        (
                            format!(
                                "{} (",
                                get_message_global(
                                    "help_text_narrator_1",
                                    props.lang.get().as_ref(),
                                    None,
                                ),
                            )
                        )
                        span(class="icon has-text-dark") { i(class="fas fa-book") }
                        (
                            format!(
                                ") {}",
                                get_message_global(
                                    "help_text_narrator_2",
                                    props.lang.get().as_ref(),
                                    None,
                                ),
                            )
                        )
                    }
                    h4 { (get_message_global("help_notes_title", props.lang.get().as_ref(), None)) }
                    ul {
                        li {
                            (
                                get_message_global("help_notes_1", props.lang.get().as_ref(), None)
                            )
                        }
                    }
                }
            }
        }
    }
}
