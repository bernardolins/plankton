pub mod operations;

mod environment;
mod status;
mod state;

use std::io::BufRead;

use serde::{Serialize, Deserialize};

use crate::error::Error;
use crate::config;

use self::status::Status;
pub use self::state::State;

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
}
