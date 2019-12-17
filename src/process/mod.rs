#[path = "linux.rs"]
#[cfg(unix)]
mod imp;

use crate::Error;
use crate::spec::ProcessSpec;

#[derive(Debug, PartialEq)]
pub struct ContainerProcess {
    inner: imp::Process,
}

impl ContainerProcess {
    pub fn from_spec<P: ProcessSpec>(proc_spec: &P) -> Result<ContainerProcess, Error> {
        Ok(ContainerProcess {
            inner: imp::Process::from_spec(proc_spec)?,
        })
    }
}

