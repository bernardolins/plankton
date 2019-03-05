extern crate serde;

use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub oci_version: String,
    pub hostname: String,
}

impl Spec {
    pub fn from_json(path: &str) -> Result<Spec, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let spec = serde_json::from_reader(reader)?;

        Ok(spec)
    }
}
