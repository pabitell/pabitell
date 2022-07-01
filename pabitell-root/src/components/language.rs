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
                Indexed {
                    iterable: languages,
                    view: move |ctx, lang| view! { ctx,
                        (
                            view! { ctx,
                                option {(lang)}
                            }
                        )
                    }
                }
            }
        }
    }
}
