pub mod linux;

use crate::Error;
use crate::process::ProcessCreate;
use crate::process::ProcessRun;
use crate::spec::ProcessSpec;

pub struct Process{}

impl ProcessCreate for Process {
    fn from_spec<P: ProcessSpec>(_spec: &P) -> Result<Process, Error> {
        Err(Error::from("platform not supported"))
    }
}

impl ProcessRun for Process {
    type PID = i32;

    fn spawn(&self) -> Result<Self::PID, Error> {
        Err(Error::from("platform not supported"))
    }

    fn exec(&self) -> Result<(), Error> {
        Err(Error::from("platform not supported"))
    }
}
