#[macro_use]
extern crate clap;
extern crate cr7;

use cr7::cli;
use cr7::Error;

use clap::App;

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        cli::run(matches)?;
    }

    Ok(())
}
