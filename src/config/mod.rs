pub mod root;
pub mod process;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

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
    root: root::Root,
    process: process::Process
}

impl Base {
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn root(&self) -> &root::Root { &self.root }
    pub fn process(&self) -> &process::Process { &self.process }
}
