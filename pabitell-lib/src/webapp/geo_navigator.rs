use geo::{coord, point, prelude::*, Line, Point};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::PositionOptions;
use yew::{html, prelude::*};

use crate::GeoLocation;

use super::Position;

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
    pub destination: Option<Point>,
    pub reached: Callback<()>,
}

const SVG_INITIAL_ROTATION: f64 = -45.;

pub struct GeoNavigator {
    pub watch_id: i32,
    #[allow(dead_code)]
    success_cb: Closure<dyn Fn(web_sys::Position)>,
    /// Distance from target
    distance: Option<f64>,
    /// Accuracy of last measurement
    accuracy: Option<f64>,
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
                    ctx.props().destination.as_ref()
                );
                let accuracy = position.accuracy;
                if let Some(destination) = ctx.props().destination.as_ref() {
                    let distance = position.point.geodesic_distance(&destination);
                    if distance < position.accuracy {
                        // Close modal
                        self.active = false;
                        ctx.props().reached.emit(());
                    } else {
                        self.update_positions(position);
                    }
                    self.distance = Some(distance);
                    self.accuracy = Some(accuracy);
                }
            }
            Msg::OpenCompass => self.active = true,
            Msg::CloseCompass => self.active = false,
        }
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let success_cb = Closure::wrap(Box::new(move |pos: web_sys::Position| {
            let position = Position {
                point: coord! {x: pos.coords().latitude(), y: pos.coords().longitude() }.into(),
                accuracy: pos.coords().accuracy(),
            };
            link.send_future(async move { Msg::LocationObtained(position) });
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
            distance: None,
            active: false,
            position1: None,
            position2: None,
            accuracy: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(destination) = ctx.props().destination.as_ref() {
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
                    let direction = Line::new(pos1.point.clone(), pos2.point.clone());
                    let rotation = Self::calculate_rotation(&destination, &direction);
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

                html! {
                    <>
                        <button
                          class="button is-outlined is-medium"
                          onclick={open_cb}
                        >
                            <span class="icon-text">
                                <span class="icon has-text-info">
                                    <i class="fas fa-compass"></i>
                                </span>
                                <span>{ distance.round() }  <small class="is-size-7 p-1 has-text-warning-dark"><i class="fas fa-plus-minus pr-1"></i>{ accuracy.round() }</small> {"m"}</span>
                            </span>
                        </button>
                        <div class={modal_classes}>
                            <div class="modal-background"></div>
                            <div class="modal-card has-text-centered">
                                <header class="modal-card-head">
                                    <p class="modal-card-title">{ distance.round() } <small class="is-size-7 p-1 has-text-warning-dark"><i class="fas fa-plus-minus pr-1"></i>{ accuracy.round() }</small> {"m"}</p>
                                </header>
                                <section class="modal-card-body">
                                    <div class="container is-flex is-justify-content-center is-align-items-center w-75">
                                        <figure class="image is-1by1 w-50">
                                            {img_html}

                                        </figure>
                                    </div>
                                </section>
                                <footer class="modal-card-foot">
                                    <a
                                      class="button"
                                      href={ Self::geo_url(destination) }
                                    >
                                        <i class="fas fa-up-right-from-square"></i>
                                    </a>
                                    <a
                                      class="button"
                                      target="_blank"
                                      href={ Self::osm_url(destination) }
                                    >
                                        <i class="fas fa-map-location"></i>
                                    </a>
                                    <a
                                      class="button"
                                      target="_blank"
                                      href={ Self::google_url(destination) }
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
        // Assign starting point
        if let Some(pos1) = self.position1.as_ref() {
            if let Some(pos2) = self.position2.as_ref() {
                if Self::more_precise(pos2, &position) {
                    // new position is more precise than the previous one
                    self.position2 = Some(position)
                } else if Self::different_positions(pos2, &position) {
                    // new location was reached - swap positions
                    self.position1 = self.position2.take();
                    self.position2 = Some(position);
                }
            } else {
                if Self::more_precise(pos1, &position) {
                    // new position is more precise than the previous one
                    self.position1 = Some(position);
                } else if Self::different_positions(pos1, &position) {
                    // second point can be established
                    self.position2 = Some(position);
                }
            }
        } else {
            // Assign starting point
            self.position1 = Some(position);
        }
    }

    fn calculate_rotation(destination: &Point, direction: &Line) -> f64 {
        let start: Point = direction.start.clone().into();
        let end: Point = direction.end.clone().into();

        SVG_INITIAL_ROTATION - start.bearing(end.clone()) + end.bearing(destination.clone())
    }

    /// Is second point more precise than the first one
    fn more_precise(p1: &Position, p2: &Position) -> bool {
        let distance = p1.point.geodesic_distance(&p2.point);
        p1.accuracy > distance + p2.accuracy
    }

    /// We are sure that the positions are different
    fn different_positions(p1: &Position, p2: &Position) -> bool {
        let distance = p1.point.geodesic_distance(&p2.point);
        distance > p1.accuracy + p2.accuracy
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
}

#[cfg(test)]
pub mod test {
    use geo::{point, Line};

    use super::{GeoNavigator, Position};

    #[test]
    fn more_precise() {
        let old = Position {
            point: point! { x: 50.0824, y: 14.5000 },
            accuracy: 30.,
        };

        // ~15m distance
        let pos1 = Position {
            point: point! { x: 50.0825, y: 14.5001 },
            accuracy: 10.,
        };
        assert_eq!(GeoNavigator::more_precise(&old, &pos1), true);
        assert_eq!(GeoNavigator::more_precise(&pos1, &old), false);

        let pos2 = Position {
            point: point! { x: 50.0825, y: 14.6000 },
            accuracy: 10.,
        };
        assert_eq!(GeoNavigator::more_precise(&old, &pos2), false);
        assert_eq!(GeoNavigator::more_precise(&pos2, &old), false);

        // ~15m distance
        let pos3 = Position {
            point: point! { x: 50.0825, y: 14.5001 },
            accuracy: 20.,
        };
        assert_eq!(GeoNavigator::more_precise(&old, &pos3), false);
        assert_eq!(GeoNavigator::more_precise(&pos3, &old), false);
    }

    #[test]
    fn different_positions() {
        let old = Position {
            point: point! { x: 50.0824, y: 14.5000 },
            accuracy: 10.,
        };
        // ~15m distance
        let pos1 = Position {
            point: point! { x: 50.0825, y: 14.5001 },
            accuracy: 5.,
        };
        assert_eq!(GeoNavigator::different_positions(&old, &pos1), true);
        assert_eq!(GeoNavigator::different_positions(&pos1, &old), true);

        // ~15m distance
        let pos1 = Position {
            point: point! { x: 50.0825, y: 14.5001 },
            accuracy: 6., // small overlap
        };
        assert_eq!(GeoNavigator::different_positions(&old, &pos1), false);
        assert_eq!(GeoNavigator::different_positions(&pos1, &old), false);
    }

    #[test]
    fn calculate_rotation() {
        let destination = point! { x: 50.0001, y: 14.5001 };

        // left
        let line1 = Line::new(
            point! { x: 50.0000, y: 14.5000 },
            point! { x: 50.0001, y: 14.5000 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line1).round(),
            -135.
        );

        // backwards
        let line2 = Line::new(
            point! { x: 50.0001, y: 14.5002 },
            point! { x: 50.0001, y: 14.5003 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line2).round(),
            135.
        );

        // right
        let line3 = Line::new(
            point! { x: 50.0000, y: 14.5000 },
            point! { x: 50.0000, y: 14.5001 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line3).round(),
            45.
        );

        // forward
        let line4 = Line::new(
            point! { x: 50.0003, y: 14.5001 },
            point! { x: 50.0002, y: 14.5001 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line4).round(),
            -45.
        );

        // upper right
        let line5 = Line::new(
            point! { x: 50.0000, y: 14.4999 },
            point! { x: 50.0000, y: 14.5000 },
        );
        assert_eq!(
            GeoNavigator::calculate_rotation(&destination, &line5).round(),
            -1.
        );
    }
}
