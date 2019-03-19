#[macro_use]
extern crate clap;
extern crate cr7;

use clap::App;
use cr7::spec;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let spec_path = matches.value_of("config").unwrap_or("config.json");

        let config = match cr7::read_config(spec_path) {
            Ok(config) => config,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        };

        match spec::build(config) {
            Ok(spec) => println!("{:?}", spec),
            Err(err) => println!("{}", err)
        }
    }
}
