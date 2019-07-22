use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::Error;
use crate::libcontainer::linux::process;
use crate::libcontainer::linux::environment::Environment;
use serde::Deserialize;
use serde::Serialize;
use failure::ResultExt;

const STATE_BASE_DIR: &str = "/run/cr7";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: String,
    init_pid: Option<i32>,
    status: Status,
    environment: Environment,
}

impl Container {
    pub fn new(container_id: &str, environment: Environment) -> Result<Container, Error> {
        let container = Container {
            id: String::from(container_id),
            init_pid: None,
            status: Status::Creating,
            environment: environment,
        };

        if state_file(container_id).exists() {
            Err(Error::from("container id already taken".to_string())).context(container_id.to_string())?;
        }

        container.save_state()?;
        Ok(container)
    }

    pub fn load(container_id: &str) -> Result<Container, Error> {
        if !state_file(container_id).exists() {
            Err(Error::from("container not found".to_string())).context(container_id.to_string())?;
        }

        let mut container = Container::load_state(container_id)?;

        if container.status != Status::Stopped {
            Err(Error::from("cannot start a non stopped container".to_string())).context(container_id.to_string())?;
        }

        container.status = Status::Created;
        container.save_state()?;

        Ok(container)
    }

    pub fn run(&mut self) -> Result<(), Error> {
        for namespace in self.environment.namespaces().as_vec() {
            namespace.enter()?;
        }

        let init_pid = process::create(&self.environment)?;

        self.init_pid = Some(init_pid);

        self.status = Status::Created;
        self.save_state()?;

        self.status = Status::Running;
        self.save_state()?;

        process::wait(init_pid)?;
        self.status = Status::Stopped;
        self.save_state()?;

        Ok(())
    }

    fn save_state(&self) -> Result<(), Error> {
        let json = serde_json::to_string(self).context("cannot save container state".to_string())?;
        fs::write(state_file(&self.id), json).context(format!("cannot save container state to file {:?}", state_file(&self.id)))?;
        Ok(())
    }

    fn load_state(container_id: &str) -> Result<Container, Error> {
        let state_file_path = state_file(container_id);
        let file = File::open(state_file_path).context("cannot open container state file".to_string())?;
        let reader = BufReader::new(file);
        let container: Container = serde_json::from_reader(reader).context("error loading container state".to_string())?;
        Ok(container)
    }
}

fn state_file(id: &str) -> PathBuf {
    let path = format!("{}/{}.json", STATE_BASE_DIR, id);
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use std::fs;
    use self::rand::Rng;

    fn setup(environment: Environment) -> Result<Container, Error> {
        let mut rng = rand::thread_rng();
        let i: i32 = rng.gen();
        let id = format!("__test__{}", i);
        Container::new(&id, environment)
    }

    fn cleanup(id: &str) {
        fs::remove_file(format!("{}/{}.json", STATE_BASE_DIR, id)).unwrap();
    }

    #[test]
    fn container_new_returns_ok_when_environment_is_valid() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let result = setup(environment);
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
        cleanup(&result.unwrap().id);
    }

    #[test]
    fn container_new_init_pid_starts_with_none() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = setup(environment).unwrap();
        assert!(container.init_pid.is_none());
        cleanup(&container.id);
    }

    #[test]
    fn container_new_status_starts_with_creating() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = setup(environment).unwrap();
        assert_eq!(container.status, Status::Creating);
        cleanup(&container.id);
    }

    #[test]
    fn container_run_sets_init_pid_when_run_is_ok() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = setup(environment).unwrap();
        assert_eq!(container.init_pid, None);
        let result = container.run();
        assert!(result.is_ok(), "expected {:?} to be ok", result);
        assert!(container.init_pid.is_some(), "expect {:?} to be Some", &container.init_pid);
        cleanup(&container.id);
    }

    #[test]
    fn container_run_sets_status_to_stopped_when_run_exits() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = setup(environment).unwrap();
        let result = container.run();
        assert!(result.is_ok(), "expected {:?} to be ok", result);
        assert_eq!(container.status, Status::Stopped);
        cleanup(&container.id);
    }

    #[test]
    fn container_load_returns_error_if_saved_state_is_not_from_a_stopped_container() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = setup(environment).unwrap();
        let result = Container::load(&container.id);
        assert!(result.is_err(), "expected {:?} to be err", result);
        cleanup(&container.id);
    }

    #[test]
    fn container_load_returns_error_if_no_state_is_found() {
        let result = Container::load("unexistent-container-id");
        assert!(result.is_err(), "expected {:?} to be err", result); }

    #[test]
    fn container_load_returns_ok_with_the_saved_container() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = setup(environment).unwrap();
        container.run().unwrap();
        let result = Container::load(&container.id);
        assert!(result.is_ok(), "expected {:?} to be err", result);
        cleanup(&container.id);
    }
}
