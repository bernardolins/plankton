use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use failure::ResultExt;
use crate::error::Error;
use crate::platform::posix;
use crate::bundle::Bundle;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub struct PosixBundle {
    path: PathBuf,
}

impl PosixBundle{
    fn load(bundle_dir: &str) -> Result<PosixBundle, Error>  {
        let bundle_path = PathBuf::from(bundle_dir);
        let path = bundle_path.canonicalize().context(format!("{:?}", bundle_path))?;
        Ok(PosixBundle{ path })
    }
}

impl Bundle for PosixBundle {
    type Config = posix::Config;

    fn check(&self) -> Result<(), Error> {
        Ok(())
    }

    fn load_config(&self) -> Result<posix::Config, Error> {
        let config_file_path = self.path.join(CONFIG_FILE_NAME);
        let config_file = File::open(&config_file_path).context(format!("{:?}", config_file_path))?;
        let config_file_reader = BufReader::new(config_file);
        let config = posix::Config::load(config_file_reader)?;
        Ok(config)
    }
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
    fn posix_bundle_load_config_return_error_when_bundle_path_does_not_exists() {
        let result = PosixBundle::load("/some/invalid/path");
        assert!(result.is_err(), "expected {:?} to be err", &result);
    }

    #[test]
    fn posix_bundle_load_config_return_error_when_config_file_is_missing() {
        let bundle = setup_bundle(None);
        let bundle_path = bundle.path().to_str().unwrap();
        let bundle = PosixBundle::load(bundle_path).unwrap();
        let result = bundle.load_config();
        assert!(result.is_err(), "expected {:?} to be err", &result);
    }

    #[test]
    fn posix_bundle_load_config_return_ok_with_a_valid_config_file() {
        let bundle = setup_bundle(Some("config.json"));
        let bundle_path = bundle.path().to_str().unwrap();
        let bundle = PosixBundle::load(bundle_path).unwrap();
        let result = bundle.load_config();
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
    }
}
