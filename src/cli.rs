use crate::Error;
use crate::bundle;
use crate::Config;
use crate::filesystem;
use crate::libcontainer::Container;
use crate::libcontainer::Environment;

use std::convert::TryFrom;

pub fn run(matches: &clap::ArgMatches) -> Result<(), Error> {
    let cwd = filesystem::cwd();
    let container_id = matches.value_of("container-id").unwrap();
    let bundle_path = matches.value_of("bundle").unwrap_or(&cwd);

    let config_file = bundle::read_config(&bundle_path)?;
    let config = Config::load(config_file)?;
    let environment = Environment::try_from(config)?;

    let mut container = Container::new(container_id, environment)?;
    container.run()?;

    Ok(())
}

pub fn start(matches: &clap::ArgMatches) -> Result<(), Error> {
    let container_id = matches.value_of("container-id").unwrap();
    Container::start(container_id)?;

    Ok(())
}

pub fn query(matches: &clap::ArgMatches) -> Result<(), Error> {
    let container_id = matches.value_of("container-id").unwrap();
    let container = Container::query(container_id)?;

    println!(
        "{0: <20} | {1: <10} | {2: <10}",
        "ID", "STATUS", "INIT PID"
    );
    println!(
        "{0: <20} | {1: <10} | {2: <10}",
        container.id(), container.status(), container.init_pid()
    );

    Ok(())
}
