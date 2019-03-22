#[cfg(target_os = "linux")]
mod linux;

use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use serde::Deserialize;

use super::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    oci_version: String,
    hostname: Option<String>,
}

impl Spec {
    pub fn new(path: &PathBuf) -> Result<Spec, Error> {
       let file = File::open(&path)?;
       let reader = BufReader::new(file);
       let spec: Spec = serde_json::from_reader(reader)?;
       Ok(spec)
    }
}
