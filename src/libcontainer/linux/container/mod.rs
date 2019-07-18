mod status;

use self::status::Status;

use crate::libcontainer::Error;
use crate::libcontainer::linux::process;
use crate::libcontainer::linux::environment::Environment;

#[derive(Debug)]
pub struct Container {
    id: String,
    init_pid: Option<i32>,
    status: Status,
    environment: Environment,
}

impl Container {
    pub fn new(container_id: &str, environment: Environment) -> Container {
        Container {
            id: String::from(container_id),
            init_pid: None,
            status: Status::Creating,
            environment: environment,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        for namespace in self.environment.namespaces().as_vec() {
            namespace.enter()?;
        }

        let init_pid = process::create(&self.environment)?;

        self.init_pid = Some(init_pid);
        self.status = Status::Created;

        process::wait(init_pid)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_new_init_pid_starts_with_none() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = Container::new("container_id", environment);
        assert!(container.init_pid.is_none());
    }

    #[test]
    fn container_new_status_starts_with_creating() {
        let environment = Environment::new(&["/bin/sh".to_string()], "rootfs");
        let container = Container::new("container_id", environment);
        assert_eq!(container.status, Status::Creating);
    }

    #[test]
    fn container_run_sets_init_pid() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = Container::new("container_id", environment);

        assert_eq!(container.init_pid, None);

        let result = container.run();

        assert!(result.is_ok(), "expected {:?} to be ok", result);
        assert!(container.init_pid.is_some(), "expect {:?} to be Some", &container.init_pid);
    }

    #[test]
    fn container_run_sets_status_to_created() {
        let environment = Environment::new(&["/usr/bin/cd".to_string(), ".".to_string()], "rootfs");
        let mut container = Container::new("container_id", environment);

        let result = container.run();

        assert!(result.is_ok(), "expected {:?} to be ok", result);
        assert_eq!(container.status, Status::Created);
    }
}
