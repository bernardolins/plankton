#[macro_use]
extern crate clap;
extern crate cr7;

use std::process;
use clap::App;
use cr7::container;
use cr7::bundle;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let bundle_path = matches.value_of("bundle").unwrap_or(".");
        let container_id = matches.value_of("container-id").unwrap();

        let bundle = match bundle::Bundle::new(bundle_path) {
            Ok(bundle) => bundle,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        };

        match container::Container::create(container_id, bundle) {
            Ok(container) => println!("{:?}", container),
            Err(err) => println!("{}", err),
        };
    }

    if let Some(matches) = matches.subcommand_matches("state") {
        let container_id = matches.value_of("container-id").unwrap();

        match container::Container::load(container_id) {
            Ok(container) => {
                if let Ok(json) = container.state().to_json() {
                    println!("{}", json);
                }

            }
            Err(err) => println!("{}", err),
        };
    }
}
