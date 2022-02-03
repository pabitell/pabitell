use sycamore::prelude::*;

use crate::translations::get_message_global;

#[derive(Prop)]
pub struct BreadCrumbProps<'a> {
    levels: &'a Signal<Vec<String>>,
}

#[component]
pub fn BreadCrumb<'a, G: Html>(ctx: ScopeRef<'a>, props: BreadCrumbProps<'a>) -> View<G> {
    view! { ctx,
        nav(class="breadcrumb", ariel-label="breadcrumbs") {
            ul {
                li {
                    a(href="/") {
                        span(class="icon"){
                            i(class="fas fa-home"){}
                        }
                    }
                }
                Indexed {
                    iterable: props.levels,
                    view: |ctx, level| view! { ctx,
                        li(class=if true {"active"} else {"false"}) {
                            a(
                                href="",
                                aria-current="page",
                                on:click=|_| {
                                    // TODO navigate to levels[:i].join()
                                    // TODO last should be kkk
                                },
                            ) {
                                (
                                    get_message_global(&level, "cs", None)
                                )
                            }
                        }
                    },
                }
            }
        }

    }
}
