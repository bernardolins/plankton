pub mod operations;

mod environment;
mod status;
mod state;

use std::io::BufRead;
use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};

use crate::error::Error;
use crate::config;

use self::status::Status;
pub use self::state::State;

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

    fn to_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    fn from_reader<R: BufRead>(reader: R) -> Result<Container, Error> {
        let container: Container = serde_json::from_reader(reader)?;
        Ok(container)
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
