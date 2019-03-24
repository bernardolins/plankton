use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";
const ROOTFS_NAME: &str = "rootfs";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    config: Config,
    path: PathBuf,
    rootfs: PathBuf,
}

impl Bundle {
    pub fn new(path: &str) -> Result<Bundle, Error> {
        let bundle_path = PathBuf::from(path).canonicalize()?;
        let config_path = bundle_path.join(CONFIG_FILE_NAME);
        let rootfs_path = bundle_path.join(ROOTFS_NAME);
        let config_file = Config::load(&config_path)?;

        let bundle = Bundle {
            config: config_file,
            path: bundle_path,
            rootfs: rootfs_path,
        };

        Ok(bundle)
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
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
