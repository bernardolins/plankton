use crate::Error;
use crate::namespace::NamespaceSet;
use crate::process::ContainerProcess;
use crate::process::ProcessCreate;
use crate::process::platform::linux::LinuxProcess;
use crate::spec::Spec;
use crate::spec::LinuxSpec;
use crate::spec::ProcessSpec;
use super::*;

#[derive(Debug, PartialEq)]
pub struct Container {
    init_process: Option<ContainerProcess>,
}

impl ContainerCreate for Container {
    fn from_spec<S: Spec>(spec: S) -> Result<Self, Error> {
        let mut container = Container{
            init_process: None,
        };

        let proc_spec = spec.get_process();
        let linux_spec = spec.get_linux();

        match proc_spec {
            Some(proc_spec) => {
                let mut ip = ContainerProcess::from_spec(proc_spec)?;
                ip.namespaces(linux_spec)?;
                container.init_process = Some(ip);
            },
            None => container.init_process = None,
        }

        Ok(container)
    }
}
