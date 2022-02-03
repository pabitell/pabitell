use sycamore_router::Route;

#[derive(Route)]
pub enum AppRoutes {
    #[not_found]
    NotFound,
    #[to("/")]
    Root,
    #[to("/<book_slug..>/")]
    Book { book_slug: Vec<String> },
}
