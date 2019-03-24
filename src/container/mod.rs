mod status;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use serde::{Serialize, Deserialize};

use crate::bundle::Bundle;
use crate::error::Error;

use self::status::Status;

const STATE_FILE_DIRECTORY: &str = "/run/cr7";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    oci_version: String,
    status: Status,
    bundle: Bundle,
}

impl Container {
    pub fn create(id: &str, bundle: Bundle) -> Result<Container, Error> {
        match Container::load_state(id) {
            Ok(_) => return Err(Error::ContainerAlreadyExists),
            Err(Error::NotFound) => (),
            Err(err) => return Err(err),
        }

        let container = Container {
            id: String::from(id),
            oci_version: String::from(""),
            status: Status::Creating,
            bundle: bundle,
        };

        container.save_state()?;

        Ok(container)
    }

    pub fn save_state(&self) -> Result<(), Error> {
        let path = Container::state_path(&self.id);
        let mut file = File::create(path)?;
        let state_string = serde_json::to_string(&self)?;
        file.write_all(state_string.as_bytes())?;
        Ok(())
    }

    pub fn load_state(container_id: &str) -> Result<Container, Error> {
        let path = Container::state_path(container_id);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let container: Container = serde_json::from_reader(reader)?;

        Ok(container)
    }

    fn state_path(container_id: &str) -> PathBuf {
        let state_path = Path::new(STATE_FILE_DIRECTORY);
        state_path.join(container_id)
    }
}
