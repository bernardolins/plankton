mod status;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use self::status::Status;

use spec::Spec;
use super::error::Error;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub struct Container {
    id: String,
    status: Status,
    bundle_path: PathBuf,
    config_path: PathBuf,
    spec: Spec,
}

impl Container {
    pub fn new(id: &str, bundle_path: &str) -> Result<Container, Error> {
        let bundle_path = PathBuf::from(bundle_path).canonicalize()?;
        let config_path = bundle_path.join(CONFIG_FILE_NAME);
        let spec = Spec::new(&config_path)?;

        let container = Container {
            id: id.to_string(),
            status: Status::Creating,
            bundle_path: bundle_path,
            config_path: config_path,
            spec: spec,
        };

        Ok(container)
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;

    use std::fs::File;

    #[test]
    fn test_bundle_path_must_exist() {
        let bundle_path = tempfile::tempdir().unwrap();
        let config_path = bundle_path.path().join(CONFIG_FILE_NAME);
        let config_file = File::create(config_path).unwrap();

        let ok = Container::new("container1", bundle_path.path().to_str().unwrap());
        assert!(ok.is_ok(), "expected {:?} to be ok", ok);

        let err = Container::new("container1", "/invalid/path/to/bundle");
        assert!(err.is_err(), "expected {:?} to be err", err);

        drop(config_file);
        bundle_path.close().unwrap();
    }
}
