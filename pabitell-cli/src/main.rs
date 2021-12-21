pub mod backend;
pub mod cli;
pub mod web;
pub mod websocket;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use pabitell_lib::{Narrator, World, WorldBuilder};

#[cfg(feature = "with_doggie_and_kitie_cake")]
fn make_story_doggie_and_kitie_cake(
    initial: bool,
) -> Result<Option<(Box<dyn World>, Box<dyn Narrator>)>> {
    let mut world: Box<dyn World> =
        Box::new(doggie_and_kitie_cake::world::CakeWorldBuilder::make_world()?);
    if initial {
        world.setup();
    }
    let mut narrator: Box<dyn Narrator> =
        Box::new(doggie_and_kitie_cake::narrator::Cake::default());

    Ok(Some((world, narrator)))
}

#[cfg(feature = "with_doggie_and_kitie_doll")]
fn make_story_doggie_and_kitie_doll(
    initial: bool,
) -> Result<Option<(Box<dyn World>, Box<dyn Narrator>)>> {
    let mut world: Box<dyn World> =
        Box::new(doggie_and_kitie_doll::world::DollWorldBuilder::make_world()?);
    if initial {
        world.setup();
    }
    let mut narrator: Box<dyn Narrator> =
        Box::new(doggie_and_kitie_doll::narrator::Doll::default());

    Ok(Some((world, narrator)))
}

fn exit_on_parse_error(mut app: App) {
    println!();
    app.write_long_help(&mut std::io::stdout()).unwrap();
    std::process::exit(1);
}

pub fn main() {
    let app = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("db-path")
                .short('P')
                .long("db-path")
                .value_name("PATH")
                .takes_value(true)
                .required(true)
                .env("PABITELL_DB_PATH"),
        )
        .subcommands(vec![
            App::new("cli").arg(
                Arg::new("default-lang")
                    .short('L')
                    .long("default-lang")
                    .value_name("LANG")
                    .takes_value(true)
                    .default_value("en-US")
                    .env("PABITELL_DEFAULT_LANG"),
            ),
            App::new("web").arg(
                Arg::new("port")
                    .env("PORT")
                    .help("Port which will be used for the web server")
                    .long("port")
                    .takes_value(true)
                    .required(false)
                    .default_value("8080")
                    .env("PABITELL_WEB_PORT"),
            ),
        ]);

    let matches = app.clone().get_matches();
    let db_path = matches.value_of("db-path").unwrap();
    match matches.subcommand() {
        Some(("web", web_matches)) => {
            let port = web_matches.value_of("port").unwrap();
            web::start_web_app(db_path, port).unwrap();
        }
        Some(("cli", cli_matches)) => {
            let default_lang = cli_matches.value_of("default-lang").unwrap();
            cli::start_cli_app(default_lang, db_path).unwrap();
        }
        _ => exit_on_parse_error(app),
    }
}
