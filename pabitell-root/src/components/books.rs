use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::data::BOOKS;
use crate::translations::get_message_global;

#[derive(Prop)]
pub struct BooksProps<'a> {
    lang: &'a Signal<String>,
    levels: &'a Signal<Vec<String>>,
}

#[component]
pub fn Books<'a, G: Html>(ctx: ScopeRef<'a>, props: BooksProps<'a>) -> View<G> {
    let books = ctx.create_signal(BOOKS.clone());
    let lang = props.lang.to_owned();
    let levels = props.levels.to_owned();
    view! { ctx,
        div(class="columns is-flex is-flex-wrap-wrap w-100") {
            Keyed {
                iterable: books,
                view: move |ctx, book| view! { ctx,
                    BookItem {
                        lang: lang,
                        levels: levels,
                        name: book.name,
                        title_slug: book.title_slug,
                        description_slug: book.description_slug,
                        img_url: book.img_url,
                    }
                },
                key: |book| book.name.to_owned(),
            }
        }
    }
}

#[derive(Prop)]
pub struct BookItemProps<'a> {
    name: String,
    title_slug: String,
    description_slug: String,
    lang: &'a Signal<String>,
    levels: &'a Signal<Vec<String>>,
    img_url: String,
}

#[component]
pub fn BookItem<'a, G: Html>(ctx: ScopeRef<'a>, props: BookItemProps<'a>) -> View<G> {
    let name = props.name.to_owned();
    view! { ctx,
        div(class="column card is-12-mobile is-6-tablet is-4-desktop is-4-widescreen is-4-fullhd m-1") {
            div(class="card-content") {
                div(class="media") {
                    div(class="media-left") {
                        figure(class="image is-48x48") {
                            img(src="images/book.svg",alt="Backpack")
                        }
                    }
                    div(class="media-content") {
                        p(class="title is-4"){(
                            get_message_global(
                                &props.title_slug, props.lang.get().as_ref(), None
                            )
                        )}
                        p(class="subtitle is-6"){}
                    }
                }
            }
            div(class="card-image has-text-centered") {
                figure(class="image is-square w-75 is-inline-block is-clickable box") {
                    img(
                        src=props.img_url,
                        on:click=move |_| {
                            props.levels.set(
                                props.levels
                                    .get()
                                    .iter()
                                    .chain([name.clone()].iter())
                                    .map(|e| e.to_owned())
                                    .collect()

                            );
                            navigate(&("/".to_owned() + &props.levels.get().join("/")));
                        },
                    ) {}
                }
            }
            div(class="card-content") {
                div(class="content") {(
                    get_message_global(
                        &props.description_slug,
                        props.lang.get().as_ref(),
                        None
                    )
                )}
            }
            footer(class="card-footer") {}
        }
    }
}
