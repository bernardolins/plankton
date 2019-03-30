use crate::bundle::Bundle;
use crate::config;
use crate::container::Container;
use crate::error::Error;

pub fn run(id: &str, bundle_path: &str) -> Result<Container, Error> {
    if Container::is_created(id) {
        return Err(Error::ContainerAlreadyExists);
    }

    let bundle = Bundle::new(bundle_path)?;
    let config = config::load(&bundle.config_path())?;

    let container = Container::new(id, bundle_path, config)?;
    container.save_on_disk()?;

    Ok(container)
}
