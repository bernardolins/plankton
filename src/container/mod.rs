mod status;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use serde::{Serialize, Deserialize};

use crate::bundle::Bundle;
use crate::error::Error;

use self::status::Status;

const CONTAINER_INFO_DIRECTORY: &str = "/run/cr7";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    oci_version: String,
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
            oci_version: String::from(""),
            status: Status::Creating,
            bundle: bundle,
        };

        container.store()?;

        Ok(container)
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
