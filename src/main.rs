#[macro_use]
extern crate clap;
extern crate cr7;

use clap::App;
use cr7::container;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    /*
     * run command
     */
    if let Some(matches) = matches.subcommand_matches("run") {
        let current_dir = str_current_dir();
        let container_id = matches.value_of("container-id").unwrap();
        let bundle_path = matches.value_of("bundle").unwrap_or(&current_dir);

        match container::operations::run(container_id, bundle_path) {
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
            _ => ()
        };
    }

    /*
     * state command
     */
    if let Some(matches) = matches.subcommand_matches("state") {
        let container_id = matches.value_of("container-id").unwrap();

        match container::operations::state(container_id) {
            Ok(state_json) => println!("{}", state_json),
            Err(err) => println!("{}", err),
        };
    }
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
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
