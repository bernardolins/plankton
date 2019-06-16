use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";

pub fn read_config(bundle_path: &str) -> Result<BufReader<File>, Error> {
    let config = config_file_path(bundle_path)?;
    let result = File::open(&config);

    if result.is_ok() {
        let file = result.unwrap();
        Ok(BufReader::new(file))
    } else {
        let file = config.to_str().unwrap_or(CONFIG_FILE_NAME);
        Err(Error::filesystem_error(file))
    }
}

fn config_file_path(path: &str) -> Result<PathBuf, Error> {
    let config_file_path = PathBuf::from(path).join(CONFIG_FILE_NAME);
    match config_file_path.canonicalize() {
        Ok(path) => Ok(path),
        Err(_) => {
            let file = config_file_path.to_str().unwrap_or(CONFIG_FILE_NAME);
            Err(Error::filesystem_error(file))
        }
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

