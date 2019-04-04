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
}
