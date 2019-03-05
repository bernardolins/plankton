extern crate clap;
extern crate cr7;

use clap::{Arg, App, SubCommand};
use cr7::Spec;

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
                    SubCommand::with_name("run")
                        .about("Creates a new container using the specification")
                        .arg(Arg::with_name("container-id")
                            .help("Creates and runs a container")
                            .required(true)
                            .index(1)
                        )
                        .arg(Arg::with_name("spec")
                            .help("The spec file path")
                            .short("s")
                            .long("spec")
                        )
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .about("Deletes a running container")
                );

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let spec_path = matches.value_of("spec").unwrap_or("config.json");
        match Spec::from_json(spec_path) {
            Ok(spec) => {
                println!("{:?}", spec);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
