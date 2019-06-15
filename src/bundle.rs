use std::io;
use std::path::PathBuf;
use crate::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";

pub fn config_file_path(path: &str) -> Result<PathBuf, Error> {
    let bundle_path = load_bundle_path(path)?;
    let config_path = load_config_file_path(bundle_path)?;
    Ok(config_path)
}

enum Reason {
    ConfigFile,
    BundlePath,
}

impl From<Reason> for Error {
    fn from(reason: Reason) -> Error {
        let message = match reason {
            Reason::ConfigFile => format!("{}: {}", CONFIG_FILE_NAME, io::Error::last_os_error()),
            Reason::BundlePath => format!("bundle: {}", io::Error::last_os_error()),
        };

        Error::new(&message)
    }
}

fn load_bundle_path(path: &str) -> Result<PathBuf, Reason> {
    match PathBuf::from(path).canonicalize() {
        Ok(path) => Ok(path),
        Err(_) => Err(Reason::BundlePath)
    }
}

fn load_config_file_path(bundle_path: PathBuf) -> Result<PathBuf, Reason> {
    match bundle_path.join(CONFIG_FILE_NAME).canonicalize() {
        Ok(path) => Ok(path),
        Err(_) => Err(Reason::ConfigFile)
    }
}

