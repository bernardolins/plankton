use std::io::BufRead;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    oci_version: String,
    hostname: Option<String>,
    root: Root,
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

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    path: String,

    #[serde(default = "Root::default_readonly")]
    readonly: bool,
}

impl Root {
    fn default_readonly() -> bool { true }

    pub fn path(&self) -> &str { &self.path }
    pub fn readonly(&self) -> bool { self.readonly}
}
