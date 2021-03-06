use crate::Error;
use crate::filesystem;
use crate::container::Container;

pub fn run(matches: &clap::ArgMatches) -> Result<(), Error> {
    let cwd = filesystem::cwd();
    let container_id = matches.value_of("container-id").unwrap();
    let bundle_dir = matches.value_of("bundle").unwrap_or(&cwd);

    Container::create(container_id, bundle_dir)?;
    Container::start(container_id)?;

    Ok(())
}

pub fn start(matches: &clap::ArgMatches) -> Result<(), Error> {
    let container_id = matches.value_of("container-id").unwrap();
    Container::start(container_id)?;

    Ok(())
}

pub fn query(matches: &clap::ArgMatches) -> Result<(), Error> {
    let container_id = matches.value_of("container-id").unwrap();
    let state = Container::state(container_id)?;
    println!("{}", state);

    Ok(())
}
