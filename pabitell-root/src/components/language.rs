use gloo::storage::{LocalStorage, Storage};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct LanguageProps<'a> {
    selected_language: &'a Signal<String>,
    languages: Vec<String>,
}

#[component]
pub fn Language<'a, G: Html>(ctx: Scope<'a>, props: LanguageProps<'a>) -> View<G> {
    let languages: Vec<String> = props
        .languages
        .clone()
        .into_iter()
        .map(|e| e.to_string())
        .collect();

    let languages = create_signal(ctx, languages);
    let selected_lang = create_selector(ctx, || props.selected_language.get().to_string());

    view! { ctx,
        div(class="select") {
            select(
                bind:value=props.selected_language,
                on:change=|_| {
                    LocalStorage::set(
                        "pabitell_lang",
                        props.selected_language.get().to_string(),
                    ).unwrap()
                },
            ) {
                option(selected=true) {
                    (props.selected_language.get())
                }
                Indexed {
                    iterable: languages,
                    view: move |ctx, lang| view! { ctx,
                        (
                            if selected_lang.get().to_string() != lang.to_string() {
                                view! { ctx,
                                    option {(lang)}
                                }
                            } else {
                                view! { ctx, }
                            }
                        )
                    }
                }
            }
        }
    }
}
