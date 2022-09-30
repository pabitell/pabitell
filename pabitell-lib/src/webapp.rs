pub mod action_event;
pub mod action_item;
pub mod action_join;
pub mod actions;
pub mod app;
pub mod character_switch;
pub mod characters;
pub mod database;
pub mod editor;
pub mod geo_navigator;
pub mod intro;
pub mod items;
#[allow(non_snake_case)]
pub mod jsQR;
pub mod language_switch;
pub mod message;
pub mod messages;
pub mod print;
pub mod qrcode;
pub mod qrscanner;
pub mod scenes;
pub mod speech;
pub mod status;
pub mod websocket_client;

use geo::Point;
use yew::Properties;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Position {
    pub point: Point,
    pub accuracy: f64,
}
