pub mod operations;

mod status;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use serde::{Serialize, Deserialize};

use crate::error::Error;
use crate::config;

use self::status::Status;

const CONTAINER_INFO_DIRECTORY: &str = "/run/cr7";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    pid: Option<i32>,
    status: Status,
    bundle_path: String,
    config: config::Base,
}

impl Container {
    fn new(id: &str, bundle_path: &str, config: config::Base) -> Result<Container, Error> {
        let container = Container {
            id: String::from(id),
            pid: None,
            status: Status::Creating,
            bundle_path: String::from(bundle_path),
            config: config,
        };

        Ok(container)
    }

    fn load_from_disk(container_id: &str) -> Result<Container, Error> {
        let path = Container::info_path(container_id);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let container: Container = serde_json::from_reader(reader)?;

        Ok(container)
    }

    fn save_on_disk(&self) -> Result<(), Error> {
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

#[derive(Serialize)]
struct State {
    oci_version: String,
    id: String,
    pid: Option<i32>,
    status: String,
    bundle: String,
}

impl From<Container> for State {
    fn from(container: Container) -> State {
        State {
            oci_version: String::from(container.config.oci_version()),
            id: String::from(container.id),
            pid: container.pid,
            status: String::from(container.status.to_str()),
            bundle: container.bundle_path,
        }
    }
}
