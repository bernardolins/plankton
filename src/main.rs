#[macro_use]
extern crate clap;
extern crate cr7;

use cr7::oci;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let spec_path = matches.value_of("spec").unwrap_or("config.json");
        match oci::load_spec(spec_path) {
            Ok(spec) => {
                println!("{:?}", spec);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
