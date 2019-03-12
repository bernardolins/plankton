#[macro_use]
extern crate clap;
extern crate cr7;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let spec_path = matches.value_of("spec").unwrap_or("config.json");

        let config = match cr7::read_config(spec_path) {
            Ok(config) => config,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        };

        match cr7::build_paltform_spec(config) {
            Ok(spec) => println!("{:?}", spec),
            Err(err) => println!("{}", err)
        }
    }
}
