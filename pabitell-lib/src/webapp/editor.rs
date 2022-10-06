use std::collections::HashMap;

use geo::{point, Point};
use url::Url;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlInputElement, PositionOptions};
use yew::{html, prelude::*};

use crate::GeoLocation;

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
    current_position: Option<(Point, f64)>,
    url_inputs: HashMap<String, Option<(Point, HtmlInputElement)>>,
}

pub enum Msg {
    UpdateSceneWithCurrentLocation(String),
    LocationObtained(Point, f64),
    RemoveLocation(String),
    UrlUpdated(String, Option<Point>, HtmlInputElement),
    ApplyUrl(String),
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
            Msg::LocationObtained(point, accuracy) => {
                log::debug!("Location obtained {:?} ~{}m", point, accuracy);
                self.current_position = Some((point, accuracy));
                true
            }
            Msg::UpdateSceneWithCurrentLocation(scene) => {
                if let Some((point, _)) = self.current_position {
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
            Msg::UrlUpdated(scene, point_opt, input_html) => {
                let value = if let Some(point) = point_opt {
                    Some((point, input_html))
                } else {
                    None
                };
                self.url_inputs.insert(scene, value);
                true
            }
            Msg::ApplyUrl(scene) => {
                if let Some((point, html_input)) =
                    self.url_inputs.insert(scene.clone(), None).unwrap()
                {
                    ctx.props()
                        .update_location
                        .emit((scene, Some(point.clone())));
                    html_input.set_value("");
                }
                true
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let success_cb = Closure::wrap(Box::new(move |pos: web_sys::Position| {
            let coords = pos.coords();
            let x = coords.latitude();
            let y = coords.longitude();
            let accuracy = coords.accuracy();
            link.send_future(async move { Msg::LocationObtained(point! {x: x, y: y}, accuracy) });
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

        let url_inputs = ctx
            .props()
            .scenes
            .iter()
            .map(|s| (s.code.clone(), None))
            .collect();

        Self {
            watch_id,
            success_cb,
            current_position: None,
            url_inputs,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let close = ctx.link().callback(|_| Msg::Close);
        let current_accuracy = self.current_position.map(|cp| cp.1);

        let render_scene_edit = |scene: &Scene| {
            let link = link.clone();
            let code = scene.code.to_string();

            let url_apply_disabled = self
                .url_inputs
                .get(&code)
                .map(|e| e.is_none())
                .unwrap_or(true);

            let code_cloned = code.clone();
            let coords_text = if let Some(geo_location) = scene.geo_location {
                format!("{} {}", geo_location.0, geo_location.1)
            } else {
                String::default()
            };

            let remove_location = link.callback(move |_| Msg::RemoveLocation(code_cloned.clone()));

            let code_cloned = code.clone();
            let set_current_location =
                link.callback(move |_| Msg::UpdateSceneWithCurrentLocation(code_cloned.clone()));

            let code_cloned = code.clone();
            let apply_url = link.callback(move |_| Msg::ApplyUrl(code_cloned.clone()));

            let update_url = link.clone().callback(move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                Msg::UrlUpdated(code.clone(), Self::parse_osm_url(&input.value()), input)
            });

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
                                <div class="field has-addons">
                                  <div class="control">
                                    <a
                                      class="button is-info is-small is-outlined"
                                      href={ Self::osm_url(self.current_position, scene.geo_location) }
                                      target="_blank"
                                      disabled={ self.current_position.is_none() }
                                    >
                                        <span class="icon is-small">
                                            <i class="fas fa-map-location"></i>
                                        </span>
                                    </a>
                                  </div>
                                  <div class="control">
                                    <input
                                      class="input is-small"
                                      type="url"
                                      placeholder="https://www.openstreetmap.org/#map=../......../........"
                                      onchange={ update_url }
                                    />
                                  </div>
                                  <div class="control">
                                    <button
                                      class="button is-success is-small is-outlined"
                                      disabled={url_apply_disabled}
                                      onclick={apply_url}
                                    >
                                        <span class="icon is-small">
                                            <i class="fas fa-check"></i>
                                        </span>
                                    </button>
                                  </div>
                                </div>
                                <div class="field">
                                    <div class="control">
                                        <button
                                          class="button is-small is-outlined is-info"
                                          onclick={ set_current_location }
                                          disabled={ current_accuracy.is_none() }
                                        >
                                            <span class="icon is-small">
                                                <i class="fas fa-arrows-to-circle"></i>
                                            </span>
                                            {
                                                if let Some(current) = current_accuracy {
                                                    html! {
                                                        <span class="ml-2">
                                                            { format!("~{}m", current.round()) }
                                                        </span>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </button>
                                    </div>
                                </div>
                            </p>
                            <p>
                                {
                                    if scene.geo_location.is_some() {
                                        html! {
                                            <div class="field has-addons">
                                                <div class="control">
                                                    <button
                                                      class="button is-small is-outlined is-danger"
                                                      onclick={remove_location}
                                                    >
                                                        <span class="icon is-small">
                                                            <i class="fas fa-trash"></i>
                                                        </span>
                                                    </button>
                                                </div>
                                                <div class="control">
                                                    <input
                                                      class="input is-small"
                                                      type="text"
                                                      placeholder="https://www.openstreetmap.org/#map=../......../........"
                                                      readonly={true}
                                                      value={ coords_text }
                                                    />
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
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

impl Editor {
    /// Build osm url
    fn osm_url(current: Option<(Point, f64)>, target: Option<GeoLocation>) -> String {
        if let Some(current) = current {
            if let Some(target) = target {
                format!(
                    "https://www.openstreetmap.org/?mlat={}&mlon={}#map=18/{}/{}",
                    target.0,
                    target.1,
                    current.0.x(),
                    current.0.y(),
                )
            } else {
                format!(
                    "https://www.openstreetmap.org/#map=18/{}/{}",
                    current.0.x(),
                    current.0.y(),
                )
            }
        } else {
            String::default()
        }
    }

    /// Parse osm url from copy paste
    fn parse_osm_url(url: &str) -> Option<Point> {
        let parsed = Url::parse(url).ok()?;
        let fragment = parsed.fragment()?;
        let map_part = fragment.splitn(2, "&").collect::<Vec<_>>()[0];
        let coords = map_part.splitn(3, "/").collect::<Vec<_>>();
        if coords.len() < 3 {
            return None;
        }
        let x: f64 = coords[1].parse().ok()?;
        let y: f64 = coords[2].parse().ok()?;

        Some(point! { x: x, y: y })
    }
}
