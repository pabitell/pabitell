use geo::{point, prelude::*, Line, Point};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Position, PositionOptions};
use yew::{html, prelude::*};

use std::{collections::HashMap, rc::Rc};

use super::characters::Character;
use crate::{translations::get_message_global, Event, GeoLocation, World};

/// Limits range for inaccurate locations
const MAX_TRIGGER_DISTANCE: f64 = 25.;
/// Extennds range when GPS is too precise
const MIN_TRIGGER_DISTANCE: f64 = 10.;
/// Can't trigger when accuracy really bad
const MAX_TRIGGER_ACCURACY: f64 = 50.;

pub type NavigationData = (Rc<Character>, Option<String>, Option<String>, Point);

impl From<Point<f64>> for GeoLocation {
    fn from(point: Point<f64>) -> Self {
        GeoLocation(point.x(), point.y())
    }
}

impl From<GeoLocation> for Point<f64> {
    fn from(geo_location: GeoLocation) -> Self {
        point! { x: geo_location.0, y: geo_location.1 }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub destination: Point,
    pub scene_name: Option<String>,
    pub scene_title: Option<String>,
    pub character: Rc<Character>,
    pub reached: Callback<(String, Point)>,
    pub lang: String,
}

const SVG_INITIAL_ROTATION: f64 = -45.;

pub struct GeoNavigator {
    pub watch_id: i32,
    #[allow(dead_code)]
    success_cb: Closure<dyn Fn(Position)>,
    current_success_cb: Option<Closure<dyn Fn(Position)>>,
    /// Distance from target
    distance: Option<f64>,
    /// Accuracy of last measurement
    accuracy: Option<f64>,
    /// Heading of last measurement
    heading: Option<f64>,
    /// Fist position for direction calculation
    position1: Option<Position>,
    /// Second position used for direction calculation
    position2: Option<Position>,
    active: bool,
}

pub enum Msg {
    LocationObtained(Position),
    OpenCompass,
    CloseCompass,
    RefreshPosition,
    SetCurrentSuccessCb(Option<Closure<dyn Fn(Position)>>),
}

impl Component for GeoNavigator {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LocationObtained(position) => {
                log::debug!(
                    "Location obtained {:?} -> {:?}",
                    position,
                    ctx.props().destination,
                );
                let accuracy = position.coords().accuracy();
                let destination = ctx.props().destination;
                let heading = position.coords().heading();

                let current_point =
                    point! { x: position.coords().latitude(), y: position.coords().longitude() };
                let distance = current_point.geodesic_distance(&destination);

                if distance
                    < f64::max(
                        MIN_TRIGGER_DISTANCE,
                        f64::min(MAX_TRIGGER_DISTANCE, accuracy),
                    )
                    && accuracy < MAX_TRIGGER_ACCURACY
                {
                    // Close modal
                    self.active = false;
                    ctx.props().reached.emit((
                        ctx.props().character.code.as_ref().clone().unwrap(),
                        destination.to_owned(),
                    ));
                } else {
                    self.update_positions(position);
                }
                self.distance = Some(distance);
                self.accuracy = Some(accuracy);
                self.heading = heading;
            }
            Msg::OpenCompass => self.active = true,
            Msg::CloseCompass => self.active = false,
            Msg::RefreshPosition => {
                if self.current_success_cb.is_some() {
                    // Already running
                    return false;
                }
                let window = web_sys::window().unwrap();
                let geolocation = window.navigator().geolocation().unwrap();

                let options = Self::make_geoapi_options();
                self.current_success_cb = Some(Self::location_obtained_cb(ctx, true));

                geolocation
                    .get_current_position_with_error_callback_and_options(
                        self.current_success_cb
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unchecked_ref(),
                        None,
                        &options,
                    )
                    .unwrap();
            }
            Msg::SetCurrentSuccessCb(current_success_cb) => {
                self.current_success_cb = current_success_cb;
            }
        }
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        let success_cb = Self::location_obtained_cb(ctx, false);
        let window = web_sys::window().unwrap();

        // prepare js instances
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().unwrap();

        let options = Self::make_geoapi_options();

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
            current_success_cb: None,
            distance: None,
            active: false,
            position1: None,
            position2: None,
            accuracy: None,
            heading: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let destination = ctx.props().destination;

        if let Some(distance) = self.distance.as_ref() {
            let accuracy = self.accuracy.unwrap();
            let modal_classes = if self.active {
                classes!("modal", "is-active")
            } else {
                classes!("modal")
            };
            let close_cb = ctx.link().callback(|_| Msg::CloseCompass);
            let open_cb = ctx.link().callback(|_| Msg::OpenCompass);

            let img_html = if let (Some(pos1), Some(pos2)) =
                (self.position1.as_ref(), self.position2.as_ref())
            {
                let direction = Line::new(
                    point! { x: pos1.coords().latitude(), y: pos1.coords().latitude() },
                    point! { x: pos2.coords().longitude(), y: pos2.coords().longitude() },
                );
                let rotation = Self::calculate_rotation(&destination, &direction, self.heading);
                html! {
                    <img
                      src="images/location-arrow.svg"
                      class="w-75"
                      style={format!("transform: rotate({}deg)", rotation)}
                    />
                }
            } else {
                html! {
                    <img
                      src="images/location-arrow.svg"
                      class="w-75 rotate"
                    />
                }
            };

            let refresh_cb = ctx.link().callback(|_| Msg::RefreshPosition);
            let refresh_disabled = self.current_success_cb.is_some();
            let refresh_classes = if refresh_disabled {
                classes!("fas", "fa-arrows-rotate", "rotate")
            } else {
                classes!("fas", "fa-arrows-rotate")
            };

            let navigate_text = get_message_global("navigate", &ctx.props().lang, None);

            html! {
                <>
                    <div class="column card is-12-mobile is-6-tablet is-3-desktop is-3-widescreen is-3-fullhd">
                        <div class="card-content">
                            <div class="media">
                                <div class="media-left">
                                    <figure class="image is-32x32">
                                        <img src="images/person-walking.svg"/>
                                    </figure>
                                </div>
                                <div class="media-content">
                                    <p class="title is-4">{ctx.props().character.short.clone()}</p>
                                    <p class="subtitle is-6">{navigate_text}</p>
                                </div>
                            </div>
                        </div>
                        <div class="card-image has-text-centered">
                            <figure onclick={ open_cb } class="image is-clickable is-square w-75 is-inline-block box">
                                <img class="box" src="images/compass.svg" alt="QR code"/>
                            </figure>
                            <div class="content">
                                <span>
                                    { distance.round() }
                                    <small class="is-size-7 p-1 has-text-warning-dark">
                                        <i class="fas fa-plus-minus pr-1"></i>
                                        { accuracy.round() }
                                    </small> {"m"}
                                </span>
                </div>
                        </div>
                    </div>
                    <div class={modal_classes}>
                        <div class="modal-background"></div>
                        <div class="modal-card has-text-centered">
                            <header class="modal-card-head">
                                <p class="modal-card-title">{ distance.round() } <small class="is-size-7 p-1 has-text-warning-dark"><i class="fas fa-plus-minus pr-1"></i>{ accuracy.round() }</small> {"m"}</p>
                                <button
                                  class="button is-info is-outlined"
                                  onclick={refresh_cb}
                                  disabled={refresh_disabled}
                                >
                                    <i class={refresh_classes}></i>
                                </button>
                            </header>
                            <section class="modal-card-body">
                                <div class="container is-flex is-justify-content-center is-align-items-center w-75">
                                    <figure class="image is-1by1 w-25">
                                        {img_html}

                                    </figure>
                                </div>
                            </section>
                            <footer class="modal-card-foot">
                                <a
                                  class="button"
                                  href={ Self::geo_url(&destination) }
                                >
                                    <i class="fas fa-up-right-from-square"></i>
                                </a>
                                <a
                                  class="button"
                                  target="_blank"
                                  href={ Self::osm_url(&destination) }
                                >
                                    <i class="fas fa-map-location"></i>
                                </a>
                                <a
                                  class="button"
                                  target="_blank"
                                  href={ Self::google_url(&destination) }
                                >
                                    <i class="fa-brands fa-google"></i>
                                </a>
                            </footer>
                        </div>
                        <button
                          onclick={close_cb}
                          class="modal-close is-large"
                          aria-label="close"
                         ></button>
                    </div>
                </>
            }
        } else {
            html! {}
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().unwrap();
        geolocation.clear_watch(self.watch_id);
    }
}

impl GeoNavigator {
    fn update_positions(&mut self, position: Position) {
        let point = point! { x: position.coords().latitude(), y: position.coords().longitude() };
        let pointa = position.coords().accuracy();
        // Assign starting point
        if let Some(pos1) = self.position1.as_ref() {
            if let Some(pos2) = self.position2.as_ref() {
                let point2 = point! { x: pos2.coords().latitude(), y: pos2.coords().longitude() };
                let point2a = pos2.coords().accuracy();
                if Self::more_precise(&point2, point2a, &point, pointa) {
                    // new position is more precise than the previous one
                    self.position2 = Some(position)
                } else if Self::different_positions(&point2, point2a, &point, pointa) {
                    // new location was reached - swap positions
                    self.position1 = self.position2.take();
                    self.position2 = Some(position);
                }
            } else {
                let point1 = point! { x: pos1.coords().latitude(), y: pos1.coords().longitude() };
                let point1a = pos1.coords().accuracy();
                if Self::more_precise(&point1, point1a, &point, pointa) {
                    // new position is more precise than the previous one
                    self.position1 = Some(position);
                } else if Self::different_positions(&point1, point1a, &point, pointa) {
                    // second point can be established
                    self.position2 = Some(position);
                }
            }
        } else {
            // Assign starting point
            self.position1 = Some(position);
        }
    }

    fn calculate_rotation(destination: &Point, direction: &Line, heading: Option<f64>) -> f64 {
        let start: Point = direction.start.clone().into();
        let end: Point = direction.end.clone().into();
        let first_angle = if let Some(heading) = heading {
            heading + 180.
        } else {
            start.bearing(end.clone())
        };

        SVG_INITIAL_ROTATION - first_angle + end.bearing(destination.clone())
    }

    /// Is second point more precise than the first one
    fn more_precise(p1: &Point, acc1: f64, p2: &Point, acc2: f64) -> bool {
        let distance = p1.geodesic_distance(&p2);
        acc1 > distance + acc2
    }

    /// We are sure that the positions are different
    fn different_positions(p1: &Point, acc1: f64, p2: &Point, acc2: f64) -> bool {
        let distance = p1.geodesic_distance(&p2);
        distance > acc1 + acc2
    }

    /// Build url in geo format
    fn geo_url(point: &Point) -> String {
        format!("geo:{},{}", point.x(), point.y())
    }

    /// Build osm url
    fn osm_url(point: &Point) -> String {
        format!(
            "https://www.openstreetmap.org/?mlat={x}&mlon={y}#map=18/{x}/{y}",
            x = point.x(),
            y = point.y(),
        )
    }

    /// Build google maps url
    fn google_url(point: &Point) -> String {
        format!(
            "https://www.google.com/maps/search/?api=1&query={x}%2C{y}",
            x = point.x(),
            y = point.y(),
        )
    }

    fn make_geoapi_options() -> PositionOptions {
        let mut options = PositionOptions::new();
        options.enable_high_accuracy(true);
        options
    }

    fn location_obtained_cb(ctx: &Context<Self>, reset_cb: bool) -> Closure<dyn Fn(Position)> {
        let link = ctx.link().clone();
        Closure::wrap(Box::new(move |pos: Position| {
            if reset_cb {
                link.send_message(Msg::SetCurrentSuccessCb(None));
            }
            link.send_future(async move { Msg::LocationObtained(pos) });
        }) as Box<dyn Fn(Position)>)
    }
}

pub fn make_navigations_data(
    events: &[Box<dyn Event>],
    world: &dyn World,
    character_map: &HashMap<String, Rc<Character>>,
) -> Vec<NavigationData> {
    events
        .iter()
        .filter_map(|e| {
            if let Some((character, scene_name_opt, location)) = e.geo_location(world) {
                if let Some(character) = character_map.get(&character) {
                    if let Some(scene_name) = scene_name_opt {
                        let scene_title = world.scenes().get(&scene_name).unwrap().short(world);
                        Some((
                            character.clone(),
                            Some(scene_name),
                            Some(scene_title),
                            point! { x: location.0, y: location.1 },
                        ))
                    } else {
                        Some((
                            character.clone(),
                            None,
                            None,
                            point! { x: location.0, y: location.1 },
                        ))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
pub mod test {
    use geo::{point, Line};

    use super::{GeoNavigator, SVG_INITIAL_ROTATION};

    #[test]
    fn more_precise() {
        let p1 = point! { x: 50.0824, y: 14.5000 };
        let p1a = 30.;

        // ~15m distance
        let p2 = point! { x: 50.0825, y: 14.5001 };
        let p2a = 10.;
        assert_eq!(GeoNavigator::more_precise(&p1, p1a, &p2, p2a), true);
        assert_eq!(GeoNavigator::more_precise(&p2, p2a, &p1, p1a), false);

        let p2 = point! { x: 50.0825, y: 14.6000 };
        let p2a = 10.;
        assert_eq!(GeoNavigator::more_precise(&p1, p1a, &p2, p2a), false,);
        assert_eq!(GeoNavigator::more_precise(&p2, p2a, &p1, p1a), false);

        // ~15m distance
        let p2 = point! { x: 50.0825, y: 14.5001 };
        let p2a = 20.;
        assert_eq!(GeoNavigator::more_precise(&p1, p1a, &p2, p2a), false,);
        assert_eq!(GeoNavigator::more_precise(&p2, p2a, &p1, p1a), false);
    }

    #[test]
    fn different_positions() {
        let old_point = point! { x: 50.0824, y: 14.5000 };
        let old_accuracy = 10.;

        // ~15m distance
        let p1 = point! { x: 50.0825, y: 14.5001 };
        let p1a = 5.;
        assert_eq!(
            GeoNavigator::different_positions(&old_point, old_accuracy, &p1, p1a),
            true
        );
        assert_eq!(
            GeoNavigator::different_positions(&p1, p1a, &old_point, old_accuracy),
            true
        );

        // ~15m distance
        let p1a = 6.; // small overlap
        assert_eq!(
            GeoNavigator::different_positions(&old_point, old_accuracy, &p1, p1a),
            false
        );
        assert_eq!(
            GeoNavigator::different_positions(&p1, p1a, &old_point, old_accuracy),
            false
        );
    }

    #[test]
    fn calculate_rotation() {
        let destination = point! { x: 50.0001, y: 14.5001 };

        // right
        let line1 = Line::new(
            point! { x: 50.0000, y: 14.5000 },
            point! { x: 50.0001, y: 14.5000 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line1, None).round(),
            SVG_INITIAL_ROTATION - 90.,
        );

        // backwards
        let line2 = Line::new(
            point! { x: 50.0001, y: 14.5002 },
            point! { x: 50.0001, y: 14.5003 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line2, None).round(),
            SVG_INITIAL_ROTATION + 180.
        );

        // left
        let line3 = Line::new(
            point! { x: 50.0000, y: 14.5000 },
            point! { x: 50.0000, y: 14.5001 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line3, None).round(),
            SVG_INITIAL_ROTATION + 90.,
        );

        // forward
        let line4 = Line::new(
            point! { x: 50.0003, y: 14.5001 },
            point! { x: 50.0002, y: 14.5001 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line4, None).round(),
            SVG_INITIAL_ROTATION,
        );

        // upper right
        let line5 = Line::new(
            point! { x: 50.0000, y: 14.4999 },
            point! { x: 50.0000, y: 14.5000 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line5, None).round(),
            -1.
        );
    }
}
