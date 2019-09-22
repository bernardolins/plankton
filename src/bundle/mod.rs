mod config;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use failure::ResultExt;
use crate::error::Error;
use crate::filesystem::pathbuf;

pub use self::config::Config;

const CONFIG_FILE_NAME: &str = "config.json";

pub trait Bundle {
    type Config;

    fn check(&self) -> Result<(), Error>;
    fn load_config(&self) -> Result<Self::Config, Error>;
}

pub fn load_config(bundle_dir: &str) -> Result<Config, Error> {
    let bundle_path = canonical_bundle_path(bundle_dir)?;
    let config_path = canonical_config_path(bundle_path)?;
    let config_reader = read_config_file(config_path)?;

    let config = Config::load(config_reader)?;
    Ok(config)
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
    use std::fs;
    use std::fs::File;
    use tempfile::{tempdir, TempDir};
    use serde_json::json;

    fn setup_bundle(config_file_name: Option<&str>) -> TempDir {
        let dir = tempdir().unwrap();
        if config_file_name.is_some() {
            let contents = json!({
                "ociVersion": "1.0.1-dev",
                "hostname": "my-container",
                "process": {
                    "terminal": true,
                    "args": ["sh"],
                    "env": ["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"],
                    "cwd": "/",
                    "rlimits": [{"type": "RLIMIT_NOFILE", "hard": 1024, "soft": 1024}],
                },
                "root": {
                    "path": "rootfs",
                    "readonly": true
                },
                "mounts": [{"destination": "/proc", "type": "proc", "source": "/proc"}],
                "linux": {
                    "namespaces": [{ "type": "pid" }]
                }
            });

            let file_path = dir.path().join(config_file_name.unwrap());
            File::create(&file_path).unwrap();
            fs::write(&file_path, serde_json::to_string(&contents).unwrap()).unwrap();
        }

        return dir;
    }

    #[test]
    fn bundle_load_config_return_error_when_bundle_path_does_not_exists() {
        let result = load_config("/some/invalid/path");
        assert!(result.is_err(), "expected {:?} to be err", &result);
    }

    #[test]
    fn bundle_load_config_return_error_when_config_file_is_missing() {
        let bundle = setup_bundle(None);
        let bundle_path = bundle.path().to_str().unwrap();
        let result = load_config(bundle_path);
        assert!(result.is_err(), "expected {:?} to be err", &result);
    }

    #[test]
    fn bundle_load_config_return_ok_with_a_valid_config_file() {
        let bundle = setup_bundle(Some("config.json"));
        let bundle_path = bundle.path().to_str().unwrap();
        let result = load_config(bundle_path);
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
    }
}
