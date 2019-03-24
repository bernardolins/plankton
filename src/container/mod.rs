mod status;
mod state;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use serde::{Serialize, Deserialize};

use crate::bundle::Bundle;
use crate::bundle::config::Config;
use crate::error::Error;

use self::status::Status;
use self::state::State;

const CONTAINER_INFO_DIRECTORY: &str = "/run/cr7";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    status: Status,
    bundle: Bundle,
}

impl Container {
    pub fn create(id: &str, bundle: Bundle) -> Result<Container, Error> {
        if Container::is_created(id) {
            return Err(Error::ContainerAlreadyExists)
        }

        let container = Container {
            id: String::from(id),
            status: Status::Creating,
            bundle: bundle,
        };

        container.store()?;

        Ok(container)
    }

    pub fn load(container_id: &str) -> Result<Container, Error> {
        let path = Container::info_path(container_id);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let container: Container = serde_json::from_reader(reader)?;

        Ok(container)
    }

    pub fn state(&self) -> State {
        State::from(self)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn config(&self) -> &Config {
        &self.bundle.config()
    }

    pub fn oci_version(&self) -> &str {
        &self.config().oci_version()
    }

    fn store(&self) -> Result<(), Error> {
        let path = Container::info_path(&self.id);
        let mut file = File::create(path)?;
        let state_string = serde_json::to_string(&self)?;
        file.write_all(state_string.as_bytes())?;
        Ok(())
    }

    fn is_created(container_id: &str) -> bool {
        let state_file = Container::info_path(container_id);
        state_file.is_file()
    }

    fn info_path(container_id: &str) -> PathBuf {
        let info_path = Path::new(CONTAINER_INFO_DIRECTORY);
        info_path.join(container_id)
    }
}
