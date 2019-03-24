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
    root: Root,
    process: Process
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Config, Error> {
       let file = File::open(&path)?;
       let reader = BufReader::new(file);
       let spec: Config = serde_json::from_reader(reader)?;
       Ok(spec)
    }

    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn root_path(&self) -> &str { &self.root.path }
    pub fn root_readonly(&self) -> bool { self.root.readonly }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    pub path: String,
    pub readonly: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Process {
    args: Vec<String>,
}
