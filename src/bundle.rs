use std::io;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";

pub fn read_config(bundle_path: &str) -> Result<BufReader<File>, Error> {
    let bundle_path = load_bundle_path(bundle_path)?;
    let config_path = load_config_file_path(bundle_path)?;

    let result = File::open(&config_path);

    if result.is_ok() {
        let file = result.unwrap();
        Ok(BufReader::new(file))
    } else {
        Err(Error::from(Reason::ConfigFile))
    }
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

fn load_bundle_path(bundle_path: &str) -> Result<PathBuf, Reason> {
    match PathBuf::from(bundle_path).canonicalize() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::{tempdir, TempDir};

    fn setup_bundle(config_file_name: Option<&str>) -> TempDir {
        let dir = tempdir().unwrap();
        if config_file_name.is_some() {
            let file_path = dir.path().join(config_file_name.unwrap());
            File::create(file_path).unwrap();
        }

        return dir;
    }

    #[test]
    fn bundle_config_file_return_error_when_bundle_path_does_not_exists() {
        let result = read_config("/some/invalid/path");
        assert!(result.is_err(), "expected {:?} to be err", &result);
    }

    #[test]
    fn bundle_config_file_return_error_when_config_file_is_missing() {
        let bundle = setup_bundle(None);
        let bundle_path = bundle.path().to_str().unwrap();
        let result = read_config(bundle_path);
        assert!(result.is_err(), "expected {:?} to be err", &result);
    }

    #[test]
    fn bundle_config_file_return_ok_with_a_bufreader_for_config_file() {
        let bundle = setup_bundle(Some("config.json"));
        let bundle_path = bundle.path().to_str().unwrap();
        let result = read_config(bundle_path);
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
    }
}

