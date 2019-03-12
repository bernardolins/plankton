use std::io::BufRead;
use std::error::Error;
use serde::Deserialize;

mod root;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    oci_version: String,
    hostname: Option<String>,
    root: root::Spec,
}

impl Spec {
    pub fn new<R: BufRead>(reader: R) -> Result<Spec, Box<Error>> {
        let spec: Spec = serde_json::from_reader(reader)?;
        Ok(spec)
    }

    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn root_path(&self) -> &str { &self.root.path() }
    pub fn is_root_readonly(&self) -> bool { self.root.readonly() }
}

