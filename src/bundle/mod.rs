pub mod config;

use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;

use self::config::Config;

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
