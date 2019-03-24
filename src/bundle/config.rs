use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    oci_version: String,
    hostname: Option<String>,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Config, Error> {
       let file = File::open(&path)?;
       let reader = BufReader::new(file);
       let spec: Config = serde_json::from_reader(reader)?;
       Ok(spec)
    }
}
