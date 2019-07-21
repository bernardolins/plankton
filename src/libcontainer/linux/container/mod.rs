use std::fs;
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

        if container.state_file().exists() {
            Err(Error::from("container id already taken".to_string())).context(container_id.to_string())?;
        }

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

    pub fn save_state(&self) -> Result<(), Error> {
        let json = serde_json::to_string(self).context("cannot save container state".to_string())?;
        fs::write(self.state_file(), json).context(format!("cannot save container state to file {:?}", self.state_file()))?;
        Ok(())
    }

    fn state_file(&self) -> PathBuf {
        let path = format!("{}/{}.json", STATE_BASE_DIR, self.id);
        PathBuf::from(path)
    }
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
        fs::remove_file(format!("{}/{}.json", STATE_BASE_DIR, id));
        Container::new(&id, environment)
    }

    fn teardown(id: &str) {
        fs::remove_file(format!("{}/{}.json", STATE_BASE_DIR, id));
    }

    #[test]
    fn container_new_returns_ok_when_environment_is_valid() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let result = setup(environment);
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
        teardown(&result.unwrap().id);
    }

    #[test]
    fn container_new_init_pid_starts_with_none() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = setup(environment).unwrap();
        assert!(container.init_pid.is_none());
        teardown(&container.id);
    }

    #[test]
    fn container_new_status_starts_with_creating() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = setup(environment).unwrap();
        assert_eq!(container.status, Status::Creating);
        teardown(&container.id);
    }

    #[test]
    fn container_run_sets_init_pid_when_run_is_ok() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = setup(environment).unwrap();
        assert_eq!(container.init_pid, None);
        let result = container.run();
        assert!(result.is_ok(), "expected {:?} to be ok", result);
        assert!(container.init_pid.is_some(), "expect {:?} to be Some", &container.init_pid);
        teardown(&container.id);
    }

    #[test]
    fn container_run_sets_status_to_stopped_when_run_exits() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = setup(environment).unwrap();
        let result = container.run();
        assert!(result.is_ok(), "expected {:?} to be ok", result);
        assert_eq!(container.status, Status::Stopped);
        teardown(&container.id);
    }
}
