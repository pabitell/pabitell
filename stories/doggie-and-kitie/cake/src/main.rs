#![recursion_limit = "4096"]

pub mod characters;
pub mod events;
pub mod items;
pub mod narrator;
pub mod scenes;
pub mod translations;
#[cfg(feature = "with_webapp")]
pub mod webapp;
pub mod world;

#[cfg(feature = "with_webapp")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "with_webapp")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    yew::start_app::<webapp::app::App>();
}

#[cfg(not(feature = "with_webapp"))]
fn main() {}
