use serde::Serialize;

use crate::container::Container;
use crate::error::Error;

#[derive(Debug, Serialize)]
pub struct State {
    pub oci_version: String,
    pub id: String,
    pub status: String,
}

impl State {
    pub fn to_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }
}

impl From<&Container> for State {
    fn from(container: &Container) -> State {
        State {
            oci_version: String::from(container.oci_version()),
            id: String::from(container.id()),
            status: String::from(container.status().to_str()),
        }
    }
}
