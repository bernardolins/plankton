use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;


pub fn load(config_path: &PathBuf) -> Result<Base, Error> {
    let file = File::open(&config_path)?;
    let reader = BufReader::new(file);
    let spec: Base = serde_json::from_reader(reader)?;
    Ok(spec)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    oci_version: String,
    hostname: Option<String>,
    root: Root,
    process: Process
}

impl Base {
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn root(&self) -> &Root { &self.root }
    pub fn process(&self) -> &Process { &self.process }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,
    pub readonly: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    args: Vec<String>,
}
