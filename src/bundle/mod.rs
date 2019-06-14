pub mod error;

use std::path::PathBuf;
pub use self::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub struct Bundle {
    config: PathBuf,
    path: PathBuf,
}

impl Bundle {
    pub fn load(path: &str) -> Result<Bundle, Error> {
        let bundle_path = PathBuf::from(path).canonicalize()?;
        let config_path = bundle_path.join(CONFIG_FILE_NAME).canonicalize()?;

        let bundle = Bundle {
            config: config_path,
            path: bundle_path,
        };

        Ok(bundle)
    }

    pub fn config_path(&self) -> &PathBuf { &self.config }
}
