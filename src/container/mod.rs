mod status;
mod error;

use container::error::Error;
use std::path::PathBuf;

use self::status::Status;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub struct Container {
    id: String,
    status: Status,
    bundle_path: PathBuf,
    config_path: PathBuf,
}

impl Container {
    pub fn new(id: &str, bundle_path: &str) -> Result<Container, Error> {
        let bundle_path = PathBuf::from(bundle_path);
        let config_path = bundle_path.join(CONFIG_FILE_NAME);

        let absolute_bundle_path = canonicalize_path(bundle_path)?;
        let absolute_config_path = canonicalize_path(config_path)?;

        let container = Container {
            id: id.to_string(),
            status: Status::Creating,
            bundle_path: absolute_bundle_path,
            config_path: absolute_config_path,
        };

        Ok(container)
    }
}

fn canonicalize_path(path: PathBuf) -> Result<PathBuf, Error> {
    match path.canonicalize() {
        Ok(path) => Ok(path),
        Err(io_error) => Err(Error::from(io_error))
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
