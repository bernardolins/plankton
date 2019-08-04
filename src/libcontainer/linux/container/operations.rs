use crate::Error;
use crate::bundle;
use crate::container::Operations;
use super::Container;
use crate::libcontainer::Environment;
use std::convert::TryFrom;

impl Operations for Container {
    fn create(container_id: &str, bundle_dir: &str) -> Result<(), Error> {
        let config = bundle::load_config(bundle_dir)?;
        let environment = Environment::try_from(config)?;
        let mut container = Container::new(container_id, environment)?;
        container.run()?;

        Ok(())
    }
}
