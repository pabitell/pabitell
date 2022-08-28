use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use super::start_cli_app;
use crate::{Narrator, World};

pub fn run<W, N, S>(story: S, world: W, narrator: N)
where
    W: World,
    N: Narrator,
    S: ToString,
{
    let app = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .args(&[Arg::new("db-path")
            .short('P')
            .long("db-path")
            .value_name("PATH")
            .takes_value(true)
            .required(true)
            .env("PABITELL_DB_PATH")]);

    let matches = app.clone().get_matches();
    let db_path = matches.value_of("db-path").unwrap();
    start_cli_app(db_path, story, world, narrator).unwrap();
}
