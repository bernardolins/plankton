use crate::Error;
use crate::spec::Spec;
use crate::spec::ProcessSpec;
use crate::process::ContainerProcess;

#[derive(Debug, PartialEq)]
pub struct Container {
    init_process: Option<ContainerProcess>,
}

impl Container {
    pub fn from_spec<S: Spec>(spec: S) -> Result<Container, Error> {
        let init_process = match spec.get_process() {
            Some(proc_spec) => Some(ContainerProcess::from_spec(proc_spec)?),
            None => None,
        };
        Ok(Container{ init_process })
    }
}
