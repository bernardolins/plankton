use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";

pub fn read_config(bundle_path: &str) -> Result<BufReader<File>, Error> {
    let config_file_path = config_file_path(bundle_path)?;

    match File::open(&config_file_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(_) => {
            let file = config_file_path.to_str().unwrap_or(CONFIG_FILE_NAME);
            Err(Error::filesystem_error(file))
        }
    }
}

fn config_file_path(path: &str) -> Result<PathBuf, Error> {
    let bundle_path = PathBuf::from(path);

    if bundle_path.is_dir() {
        let config_file_path = bundle_path.join(CONFIG_FILE_NAME);
        if let Ok(path) = config_file_path.canonicalize() {
            Ok(path)
        } else {
            let file = config_file_path.to_str().unwrap_or(CONFIG_FILE_NAME);
            Err(Error::filesystem_error(file))
        }
    } else {
        Err(Error::filesystem_error(path))
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

