pub mod root;
pub mod process;
pub mod mount;
pub mod error;

#[cfg(target_os = "linux")]
pub mod linux;

mod conv;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

pub use self::error::Error;

pub use self::linux::Namespace;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    oci_version: String,
    hostname: Option<String>,
    root: root::Root,
    process: process::Process,
    mounts: Vec<mount::Mount>,

    #[cfg(target_os = "linux")] linux: linux::Linux,
}

impl Config {
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn root(&self) -> &root::Root { &self.root }
    pub fn process(&self) -> &process::Process { &self.process }
    pub fn mounts(&self) -> &Vec<mount::Mount> { &self.mounts }


    #[cfg(target_os = "linux")]
    pub fn namespaces(&self) -> &Vec<Namespace> {
        &self.linux.namespaces
    }

    pub fn load(config_path: &PathBuf) -> Result<Config, Error> {
        let file = File::open(&config_path)?;
        let reader = BufReader::new(file);
        let spec: Config = serde_json::from_reader(reader)?;
        Ok(spec)
    }
}
