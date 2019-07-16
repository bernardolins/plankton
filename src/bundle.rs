use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use failure::ResultExt;
use crate::error::Error;
use crate::filesystem::pathbuf;

const CONFIG_FILE_NAME: &str = "config.json";

pub fn read_config(bundle_dir: &str) -> Result<BufReader<File>, Error> {
    let bundle_path = canonical_bundle_path(bundle_dir)?;
    let config_path = canonical_config_path(bundle_path)?;

    read_config_file(config_path)
}

fn canonical_bundle_path(bundle_dir: &str) -> Result<PathBuf, Error> {
    let bundle_path = PathBuf::from(bundle_dir);
    let path = bundle_path.canonicalize().context(pathbuf::to_string(bundle_path))?;
    Ok(path)
}

fn canonical_config_path(bundle_path: PathBuf) -> Result<PathBuf, Error> {
    let config_file_path = bundle_path.join(CONFIG_FILE_NAME);
    let path = config_file_path.canonicalize().context(pathbuf::to_string(config_file_path))?;
    Ok(path)
}

fn read_config_file(path: PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(&path).context(pathbuf::to_string(path))?;
    Ok(BufReader::new(file))
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

