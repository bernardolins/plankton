#[macro_use]
extern crate clap;
extern crate cr7;

use clap::App;

use cr7::Error;
use cr7::Config;
use cr7::bundle;
use cr7::libcontainer::Container;
use cr7::libcontainer::Environment;

use std::convert::TryFrom;

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let current_dir = str_current_dir();
        let container_id = matches.value_of("container-id").unwrap();
        let bundle_path = matches.value_of("bundle").unwrap_or(&current_dir);

        let config_file_path = bundle::config_file_path(&bundle_path)?;

        let config = Config::load(&config_file_path).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(2);
        });

        let environment = Environment::try_from(config).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(3);
        });


        let mut container = Container::new(container_id, environment);
        container.run().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(4);
        });
    }

    Ok(())
}

fn str_current_dir() -> String {
    match std::env::current_dir() {
        Ok(current_path) => {
            match current_path.to_str() {
                Some(dir) => String::from(dir),
                None => String::from("./")
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
