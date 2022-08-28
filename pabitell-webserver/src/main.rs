pub mod web;
pub mod websocket;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn main() {
    let app = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("port")
                .env("PORT")
                .help("Port which will be used for the web server")
                .long("port")
                .takes_value(true)
                .required(false)
                .default_value("8080")
                .env("PABITELL_WEB_PORT"),
        );

    let matches = app.clone().get_matches();
    let port = matches.value_of("port").unwrap();
    web::start_web_app(port).unwrap();
}
