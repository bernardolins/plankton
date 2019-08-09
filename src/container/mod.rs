pub mod state;
pub mod status;

pub use self::state::State;
pub use self::status::Status;

use crate::bundle;
use crate::Error;
use crate::libcontainer::Environment;
use failure::ResultExt;
use serde::Serialize;
use serde::Deserialize;
use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const CONTAINER_DIR: &str = "/run/cr7";

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    pid: Option<i32>,
    status: Status,
    bundle: PathBuf,
}

impl Container {
    pub fn create(container_id: &str, bundle_dir: &str) -> Result<(), Error> {
        if Container::file_path(container_id).exists() {
            Err(Error::from("container id already taken".to_string())).context(container_id.to_string())?;
        }

        let config = bundle::load_config(bundle_dir)?;
        let environment = Environment::try_from(config)?;

        let mut container = Container {
            id: String::from(container_id),
            bundle: PathBuf::from(bundle_dir),
            status: Status::Creating,
            pid: None,
        };
        container.save()?;

        let init_pid = environment.spawn_process()?;
        container.pid = Some(init_pid);
        container.status = Status::Created;
        container.save()?;

        container.status = Status::Running;
        container.save()?;

        Environment::wait_process(init_pid)?;
        container.status = Status::Stopped;
        container.save()?;

        Ok(())
    }

    pub fn state(container_id: &str) -> Result<String, Error> {
        let container = Container::load(container_id)?;
        let json = serde_json::to_string_pretty(&container).context("cannot save container state".to_string())?;
        Ok(json)
    }

    fn save(&self) -> Result<(), Error> {
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

    fn file_path(container_id: &str) -> PathBuf {
        let path = format!("{}/{}.json", CONTAINER_DIR, container_id);
        PathBuf::from(path)
    }
}
