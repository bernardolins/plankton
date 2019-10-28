pub mod state;
pub mod status;
pub mod platform;

pub use self::state::State;
pub use self::status::Status;

use crate::Error;
use crate::bundle::Bundle;
use crate::spec::Spec;
use crate::libcontainer::Environment;
use failure::ResultExt;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::marker::PhantomData;
use std::io::BufReader;
use std::path::PathBuf;

pub trait ContainerRunner {
    fn run_entrypoint(&mut self) -> Result<(), Error>;
}

pub trait ContainerBuilder {
    type Spec: Spec;

    fn from_bundle(id: &str, bundle: Bundle<Self::Spec>) -> Result<Self, Error> where Self: Sized;
}

pub trait ContainerInfo {
    fn exists(id: &str) -> bool;
    fn current_state(id: &str) -> Result<State, Error>;
    fn update_state(id: &str, new_state: State) -> Result<(), Error>;
}

pub struct ContainerOps<P: ContainerBuilder + ContainerRunner + ContainerInfo> {
   platform: PhantomData<P>,
}

impl <P: ContainerBuilder + ContainerRunner + ContainerInfo> ContainerOps<P> {
    pub fn run(_container_id: &str, _bundle_dir: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn start(_container_id: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn query(_container_id: &str) -> Result<(), Error> {
        Ok(())
    }
}

const CONTAINER_DIR: &str = "/run/plankton";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    bundle: String,
    id: String,
    pid: Option<i32>,
    status: Status,
}

impl Container {
    pub fn create(container_id: &str, bundle_dir: &str) -> Result<(), Error> {
        if Container::file_path(container_id).exists() {
            Err(Error::from("container id already taken".to_string())).context(container_id.to_string())?;
        }

        let container = Container {
            id: String::from(container_id),
            bundle: String::from(bundle_dir),
            status: Status::Creating,
            pid: None,
        }; container.save()?;

        Ok(())
    }

    pub fn start(container_id: &str) -> Result<(), Error> {
        let mut container = Container::load(container_id)?;

        if container.status != Status::Creating && container.status != Status::Stopped {
            Err(Error::from("cannot start a non stopped container".to_string())).context(container_id.to_string())?;
        }
        let environment = Environment::build(&container.bundle)?;

        let spawn_result = environment.spawn_process();
        if spawn_result.is_err() {
            Container::delete(container_id)?;
        }

        container.update_status(Status::Created)?;

        let init_pid = spawn_result.ok().unwrap();
        container.pid = Some(init_pid);

        container.update_status(Status::Running)?;

        let wait_result = Environment::wait_process(init_pid);
        if wait_result.is_err() {
            Container::delete(container_id)?;
        }
        container.update_status(Status::Stopped)?;

        Ok(())
    }

    pub fn state(container_id: &str) -> Result<String, Error> {
        let container = Container::load(container_id)?;
        let json = serde_json::to_string_pretty(&container).context("cannot save container state".to_string())?;
        Ok(json)
    }

    fn update_status(&mut self, status: Status) -> Result<(), Error> {
        self.status = status;
        self.save()?;
        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        if !PathBuf::from(CONTAINER_DIR).exists() {
            fs::create_dir_all(CONTAINER_DIR).context(format!("error creating state dir {}", CONTAINER_DIR))?;
        }
        let file = Container::file_path(&self.id);
        let json = serde_json::to_string(self).context("cannot save container state".to_string())?;
        fs::write(&file, json).context(format!("cannot save container state to file {:?}", &file))?;
        Ok(())
    }

    fn load(container_id: &str) -> Result<Container, Error> {
        let container_file = Container::file_path(container_id);
        if !container_file.exists() {
            Err(Error::from("container not found".to_string())).context(container_id.to_string())?;
        }
        let file = File::open(container_file).context("cannot open container state file".to_string())?;
        let reader = BufReader::new(file);
        let container: Container = serde_json::from_reader(reader).context("error loading container state".to_string())?;
        Ok(container)
    }

    fn delete(container_id: &str) -> Result<(), Error> {
        let file_path = Container::file_path(container_id);
        fs::remove_file(file_path).context("error deleting container state file".to_string())?;
        Ok(())
    }

    fn file_path(container_id: &str) -> PathBuf {
        let path = format!("{}/{}.json", CONTAINER_DIR, container_id);
        PathBuf::from(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::fs::File;
    use tempfile::{tempdir, TempDir};

    fn setup_bundle(config_file_name: Option<&str>) -> TempDir {
        let dir = tempdir().unwrap();
        if config_file_name.is_some() {
            let contents = json!({
                "ociVersion": "1.0.1-dev",
                "hostname": "my-container",
                "process": {
                    "terminal": true,
                    "args": ["cd", "."],
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
                    "namespaces": [{ "type": "pid" },{ "type": "uts" }]
                }
            });

            let rootpath = dir.path().join(PathBuf::from("rootfs"));
            fs::create_dir_all(rootpath).unwrap();
            let file_path = dir.path().join(config_file_name.unwrap());
            File::create(&file_path).unwrap();
            fs::write(&file_path, serde_json::to_string(&contents).unwrap()).unwrap();
        }

        return dir;
    }

    #[test]
    fn container_create_returns_error_if_container_already_exist() {
        let container_id = "my-container-id";
        let container = Container {
            id: String::from(container_id),
            bundle: String::from("/containers/mycontainer"),
            status: Status::Creating,
            pid: Some(5327),
        };
        container.save().unwrap();

        let bundle = setup_bundle(Some("config.json"));
        let bundle_path = bundle.path().to_str().unwrap();
        let result = Container::create(container_id, bundle_path);
        assert!(result.is_err());
        Container::delete(container_id).unwrap();
    }

    #[test]
    fn container_create_returns_ok_when_container_finish_to_run() {
        let container_id = "my-container-id";
        let bundle = setup_bundle(Some("config.json"));
        let bundle_path = bundle.path().to_str().unwrap();
        let result = Container::create(container_id, bundle_path);
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
    }

    #[test]
    fn container_state_returns_error_when_container_is_not_found() {
        let result = Container::state("unexistent-containter");
        assert!(result.is_err());
    }

    #[test]
    fn container_state_returns_the_json_version_of_container() {
        let container_id = "my-container-id";
        let container = Container {
            id: String::from(container_id),
            bundle: String::from("/containers/mycontainer"),
            status: Status::Creating,
            pid: Some(5327),
        };

        container.save().unwrap();

        let json_state = json!({
            "id": "my-container-id",
            "status": "Creating",
            "pid": Some(5327),
            "bundle": "/containers/mycontainer",
        });

        let result = Container::state(container_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), serde_json::to_string_pretty(&json_state).unwrap());
        Container::delete(container_id).unwrap();
    }
}
