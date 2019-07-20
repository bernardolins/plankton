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

    let mut container = Container::new(container_id, environment);
    container.run()?;

    Ok(())
}
