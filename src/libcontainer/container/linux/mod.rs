mod process;
mod namespace;

use crate::libcontainer::container::status::Status;
use crate::libcontainer::environment::Environment;

#[derive(Debug)]
pub struct Container {
    init_pid: Option<i32>,
    status: Status,
    environment: Environment,
}

impl Container {
    pub fn new(environment: Environment) -> Container {
        Container {
            init_pid: None,
            status: Status::Creating,
            environment: environment,
        }
    }

    pub fn run(&mut self) {
        let init_pid = process::create(&self.environment);

        self.init_pid = Some(init_pid);
        self.status = Status::Created;

        process::wait(init_pid);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_new_init_pid_starts_with_none() {
        let environment = Environment::new(&["/bin/sh"], "rootfs");
        let container = Container::new(environment);
        assert!(container.init_pid.is_none());
    }

    #[test]
    fn container_new_status_starts_with_creating() {
        let environment = Environment::new(&["/bin/sh"], "rootfs");
        let container = Container::new(environment);
        assert_eq!(container.status, Status::Creating);
    }

    #[test]
    fn container_run_sets_init_pid() {
        let environment = Environment::new(&["/usr/bin/cd", "."], "rootfs");
        let mut container = Container::new(environment);

        assert_eq!(container.init_pid, None);

        container.run();

        assert!(container.init_pid.is_some(), "expect {:?} to be Some", &container.init_pid);
    }

    #[test]
    fn container_run_sets_status_to_created() {
        let environment = Environment::new(&["/usr/bin/cd", "."], "rootfs");
        let mut container = Container::new(environment);

        container.run();

        assert_eq!(container.status, Status::Created);
    }
}
