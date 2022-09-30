use geo::{point, Point};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::PositionOptions;
use yew::{html, prelude::*};

use super::scenes::Scene;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub places_text: String,
    pub scenes: Vec<Scene>,
    pub close: Callback<()>,
    pub update_location: Callback<(String, Option<Point>)>,
}

pub struct Editor {
    watch_id: i32,
    #[allow(dead_code)]
    success_cb: Closure<dyn Fn(web_sys::Position)>,
    current_position: Option<Point>,
}

pub enum Msg {
    UpdateSceneWithCurrentLocation(String),
    LocationObtained(Point),
    RemoveLocation(String),
    Close,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                ctx.props().close.emit(());
                true
            }
            Msg::LocationObtained(point) => {
                self.current_position = Some(point);
                true
            }
            Msg::UpdateSceneWithCurrentLocation(scene) => {
                if let Some(point) = self.current_position {
                    ctx.props().update_location.emit((scene, Some(point)));
                    true
                } else {
                    log::warn!("Current position is not known yet!");
                    true
                }
            }
            Msg::RemoveLocation(scene) => {
                ctx.props().update_location.emit((scene, None));
                true
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let success_cb = Closure::wrap(Box::new(move |pos: web_sys::Position| {
            let x = pos.coords().latitude();
            let y = pos.coords().longitude();
            link.send_future(async move { Msg::LocationObtained(point! {x: x, y: y}) });
        }) as Box<dyn Fn(web_sys::Position)>);

        let window = web_sys::window().unwrap();
        // prepare js instances
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().unwrap();

        let mut options = PositionOptions::new();
        options.enable_high_accuracy(true);

        let watch_id = geolocation
            .watch_position_with_error_callback_and_options(
                success_cb.as_ref().unchecked_ref(),
                None,
                &options,
            )
            .unwrap();

        Self {
            watch_id,
            success_cb,
            current_position: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let close = ctx.link().callback(|_| Msg::Close);
        let current_disabled = self.current_position.is_none();

        let render_scene_edit = |scene: &Scene| {
            let code = scene.code.to_string();
            let code_cloned = code.clone();
            let coords_html = if let Some(geo_location) = scene.geo_location {
                html! {
                    <>
                        <small>{ format!("Lat {}", geo_location.0) }</small>
                        <small class="ml-2">{ format!("Lon {}", geo_location.1) }</small>
                    </>
                }
            } else {
                html! {}
            };

            let set_current_location = link
                .clone()
                .callback(move |_| Msg::UpdateSceneWithCurrentLocation(code.clone()));

            let remove_location = link
                .clone()
                .callback(move |_| Msg::RemoveLocation(code_cloned.clone()));

            html! {
                <article class="media">
                    <figure class="media-left">
                        <p class="image is-64x64">
                            <img src={ scene.image_url.clone() } />
                        </p>
                    </figure>
                    <div class="media-content">
                        <div class="content">
                            <p>
                                <strong>{ &scene.short }</strong>
                            </p>
                            <p>
                                <button class="button is-inverted is-small">
                                    <span class="icon is-small">
                                        <i class="fas fa-location-crosshairs"></i>
                                    </span>
                                    { coords_html }
                                </button>
                                {
                                    if scene.geo_location.is_some() {
                                        html! {
                                            <button class="button is-small is-outlined is-danger" onclick={remove_location}>
                                                <span class="icon is-small">
                                                    <i class="fas fa-trash"></i>
                                                </span>
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                <button
                                  class="button is-small is-outlined is-info"
                                  onclick={ set_current_location }
                                  disabled={ current_disabled }
                                >
                                    <span class="icon is-small">
                                        <i class="fas fa-arrows-to-circle"></i>
                                    </span>
                                </button>
                            </p>
                        </div>
                    </div>
                </article>
            }
        };

        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{ &ctx.props().title }</p>
                        <button
                          class="delete"
                          aria-label="close"
                          onclick={close}
                        ></button>
                    </header>
                    <section class="modal-card-body">
                        <div class="content">
                            <h3>{ &ctx.props().places_text }</h3>
                            { for ctx.props().scenes.iter().map(render_scene_edit) }
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                    </footer>
                </div>
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().unwrap();
        geolocation.clear_watch(self.watch_id);
    }
}

impl Editor {}
