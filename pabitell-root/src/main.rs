mod components;
mod data;
mod router;
mod translations;

use gloo::storage::{LocalStorage, Storage};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use components::{
    book::Book, books::Books, breadcrumb::BreadCrumb, footer::Footer, help::Help,
    language::Language, title::Title,
};
use data::{BOOKS, LANGUAGES};
use router::AppRoutes;
use translations::get_message_global;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|ctx| {
        let lang: String = (LocalStorage::get("pabitell_lang") as Result<String, _>)
            .unwrap_or_else(|_| "en".to_string());
        let lang = create_signal(ctx, lang);
        view! { ctx,
            Router {
                integration: HistoryIntegration::new(),
                view: move |ctx, route: &ReadSignal<AppRoutes>| {
                    let levels = create_signal(ctx, vec![] as Vec<String>);
                    let not_found_text = get_message_global("not_found", &lang.get(), None);
                    view! { ctx,
                        div(class="root") {
                            div(class="section page-header pb-1 pt-0") {
                                Title {
                                    selected_language: lang,
                                }
                                div(class="content is-flex is-justify-content-center") {
                                    Help { lang }
                                    Language {
                                        selected_language: lang,
                                        languages: LANGUAGES.iter().map(|e| e.to_string()).collect(),
                                    }
                                }
                                BreadCrumb {
                                    levels: levels,
                                    selected_language: lang,
                                }
                            }
                            main(class="section is-flex") {
                                (
                                    match route.get().as_ref() {
                                        AppRoutes::NotFound => view! { ctx,
                                            div(class="box notification is-warning") {
                                                span(class="icon") {
                                                    i(class="fas fa-exclamation"){}
                                                }
                                                strong{(not_found_text)}
                                            }
                                        },
                                        AppRoutes::Root => {
                                            levels.set(vec![]);
                                            view! { ctx,
                                                Books{
                                                    lang: lang,
                                                    levels: levels,
                                                }
                                            }
                                        },
                                        AppRoutes::Book {book_slug} => {
                                            let book_slug = book_slug.to_owned();

                                            let matches = BOOKS.iter().filter(|b| b.name == book_slug[0]).collect::<Vec<_>>();
                                            levels.set(book_slug);

                                            if !matches.is_empty() {

                                                view! { ctx,
                                                    Book {
                                                        lang: lang,
                                                        book: matches[0].clone(),
                                                    }
                                                }
                                            } else {
                                                view! { ctx,
                                                    div(class="box notification is-warning w-100") {
                                                        span(class="icon") {
                                                            i(class="fas fa-exclamation"){}
                                                        }
                                                        strong{(not_found_text)}
                                                    }
                                                }
                                            }
                                        },
                                    }
                                )
                            }
                            Footer {}
                        }
                    }
                },

            }
        }
    });
}
