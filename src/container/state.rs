use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Write;
use serde::Serialize;
use crate::error::Error;
use crate::container::Container;

const STATE_FILE_DIRECTORY: &str = "/run/cr7";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    oci_version: String,

    id: String,

    status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<String>,
}

impl State {
    pub fn new(container: &Container) -> Result<State, Error> {
        State::check_existence(container.id())?;

        let state = State {
            oci_version: String::from(container.oci_version()),
            id: String::from(container.id()),
            status: String::from(container.current_status()),
            pid: None,
        };

        Ok(state)
    }

    pub fn save(&self) -> Result<(), Error> {
        let file_path = State::file_path(&self.id);
        let mut file = File::create(file_path)?;
        let state_string = serde_json::to_string(&self)?;
        file.write_all(state_string.as_bytes())?;
        Ok(())
    }

    pub fn check_existence(container_id: &str) -> Result<(), Error> {
        match State::file_path(container_id).is_file() {
            true => Err(Error::ContainerAlreadyExists),
            false => Ok(()),

        }
    }

    fn file_path(container_id: &str) -> PathBuf {
        let state_path = Path::new(STATE_FILE_DIRECTORY);
        state_path.join(container_id)
    }
}
