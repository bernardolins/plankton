extern crate clap;
use clap::{Arg, App, SubCommand};

const VERSION: &str = "0.0.1";
const APP_NAME: &str = "cr7";


fn main() {
    let app = App::new(APP_NAME)
                .version(VERSION)
                .about("Container runtime")
                .arg(
                    Arg::with_name("version")
                        .short("v")
                )
                .subcommand(
                    SubCommand::with_name("init")
                        .about("Creates a new specification file")
                )
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Creates a new container using the specification")
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .about("Delestes a running container")
                );

    let _matches = app.get_matches();
}
