mod status;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use spec::Spec;
use super::error::Error;
use self::status::Status;
use serde::{Serialize, Deserialize};

const CONFIG_FILE_NAME: &str = "config.json";
const STATE_FILE_DIRECTORY: &str = "/run/cr7";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    oci_version: String,
    status: Status,
    bundle_path: PathBuf,
}

impl Container {
    pub fn id(&self) -> &str { &self.id }
    pub fn current_status(&self) -> &str { &self.status.to_str() }
    pub fn oci_version(&self) -> &str { &self.oci_version }

    pub fn create(id: &str, bundle_path: &str) -> Result<Container, Error> {
        match Container::load_state(id) {
            Ok(_) => return Err(Error::ContainerAlreadyExists),
            Err(Error::NotFound) => (),
            Err(err) => return Err(err),
        }

        let bundle_path = PathBuf::from(bundle_path).canonicalize()?;
        let config_path = bundle_path.join(CONFIG_FILE_NAME);
        let spec = Spec::new(&config_path)?;

        let container = Container {
            id: String::from(id),
            oci_version: String::from(spec.oci_version()),
            status: Status::Creating,
            bundle_path: bundle_path,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn container() -> Container {
        Container {
            id: String::from("container1"),
            oci_version: String::from("1.0.0"),
            status: Status::Creating,
            bundle_path: PathBuf::from("/containers/container1"),
        }
    }

    #[test]
    fn id() {
        assert_eq!(container().id(), "container1");
    }

    #[test]
    fn current_status() {
        assert_eq!(container().current_status(), Status::Creating.to_str());
    }

    #[test]
    fn oci_version() {
        assert_eq!(container().oci_version(), "1.0.0");
    }
}
