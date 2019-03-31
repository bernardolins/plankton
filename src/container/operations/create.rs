use crate::bundle::Bundle;
use crate::config;
use crate::container::Container;
use crate::container::Status;
use crate::container::environment;
use crate::error::Error;

pub fn run(container_id: &str, bundle_path: &str) -> Result<Container, Error> {
    let container = environment::load(container_id)?;

    if let Some(_) = container {
        return Err(Error::ContainerAlreadyExists)
    }

    let bundle = Bundle::new(bundle_path)?;
    let config = config::load(&bundle.config_path())?;

    let mut container = Container::new(container_id, bundle_path, config)?;

    environment::save(&container)?;

    container.set_pid(nix::unistd::getpid().as_raw());
    container.set_status(Status::Created);
    environment::save(&container)?;

    Ok(container)
}
