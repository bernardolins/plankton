mod status;
mod state;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use spec::Spec;
use super::error::Error;
use self::status::Status;
use self::state::State;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub struct Container {
    id: String,
    oci_version: String,
    status: Status,
    bundle_path: PathBuf,
}

impl Container {
    pub fn id(&self) -> &str { &self.id }
    pub fn current_status(&self) -> &str { &self.status.to_str() }
    pub fn oci_version(&self) -> &str { &self.oci_version }

    pub fn new(id: &str, bundle_path: &str) -> Result<Container, Error> {
        let bundle_path = PathBuf::from(bundle_path).canonicalize()?;
        let config_path = bundle_path.join(CONFIG_FILE_NAME);
        let spec = Spec::new(&config_path)?;

        let container = Container {
            id: String::from(id),
            oci_version: String::from(spec.oci_version()),
            status: Status::Creating,
            bundle_path: bundle_path,
        };

        let state = State::new(&container)?;
        state.save()?;

        Ok(container)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn container() -> Container {
        Container {
            id: String::from("container1"),
            oci_version: String::from("1.0.0"),
            status: Status::Creating,
            bundle_path: PathBuf::from("/containers/container1"),
        }
    }

    #[test]
    fn id() {
        assert_eq!(container().id(), "container1");
    }

    #[test]
    fn current_status() {
        assert_eq!(container().current_status(), Status::Creating.to_str());
    }

    #[test]
    fn oci_version() {
        assert_eq!(container().oci_version(), "1.0.0");
    }
}
