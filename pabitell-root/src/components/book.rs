use qrcode::{render::svg, EcLevel, QrCode};
use sycamore::prelude::*;
use web_sys::window;

use crate::data::{Book, Chapter};
use crate::translations::get_message_global;

#[derive(Prop)]
pub struct BookProps<'a> {
    lang: &'a Signal<String>,
    book: Book,
}

#[component]
pub fn Book<'a, G: Html>(ctx: Scope<'a>, props: BookProps<'a>) -> View<G> {
    let chapters = create_signal(ctx, props.book.chapters.clone());
    let lang = props.lang.to_owned();
    view! { ctx,
        div(class="content w-100") {
            h2{ (get_message_global(&props.book.title_slug, &props.lang.get(), None)) }
            p{ (get_message_global(&props.book.description_slug, &props.lang.get(), None)) }
            div(class="columns is-flex is-flex-wrap-wrap w-100") {
                Keyed {
                    iterable: chapters,
                    view: move |ctx, chapter| view! { ctx,
                        Chapter {
                            lang: lang,
                            chapter: chapter,
                        }
                    },
                    key: |chapter| chapter.name.to_owned(),
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct ChapterProps<'a> {
    lang: &'a Signal<String>,
    chapter: Chapter,
}

#[component]
pub fn Chapter<'a, G: Html>(ctx: Scope<'a>, props: ChapterProps<'a>) -> View<G> {
    // Prepare qrcode with url
    let location = window().unwrap().location();
    let url = format!("{}/{}", location.href().unwrap(), &props.chapter.name);

    let qrcode = QrCode::with_error_correction_level(url, EcLevel::H).unwrap();
    let img = qrcode
        .render()
        .min_dimensions(250, 250)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    let show_qr = create_signal(ctx, false);

    view! { ctx,
        div(class="column card is-12-mobile is-6-tablet is-4-desktop is-4-widescreen is-4-fullhd m-1") {
            div(class="card-content") {
                div(class="media") {
                    div(class="media-content") {
                        p(class="title is-4"){(
                            get_message_global(
                                &props.chapter.title_slug, &props.lang.get(), None
                            )
                        )}
                        p(class="subtitle is-6"){}
                    }
                }
            }
            div(class="card-image has-text-centered") {
                figure(class="image is-square w-75 is-inline-block is-clickable box") {
                    a(href=props.chapter.target_url, rel="external") {
                        img(src=props.chapter.img_url) {}
                    }
                }
            }
            div(class="card-content") {
                div(class="content") {(
                    get_message_global(
                        &props.chapter.description_slug,
                        props.lang.get().as_ref(),
                        None
                    )
                )}
            }
            footer(class="card-footer") {
                button(
                    class="button is-large card-footer-item",
                    on:click=|_| { show_qr.set(true) }
                ) {
                    i(class="fas fa-qrcode") {}
                }
            }
            div(class=if *show_qr.get() { "modal is-active" } else { "modal" }) {
                div(class="modal-background")
                div(class="modal-content has-text-centered") {
                    div(dangerously_set_inner_html=&img)
                }
                button(
                    class="modal-close is-large delete",
                    aria-label="close",
                    on:click=|_| { show_qr.set(false) }
                )
            }
        }
    }
}
