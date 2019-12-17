pub mod linux;

use crate::Error;
use crate::process::ProcessCreate;
use crate::spec::ProcessSpec;

pub struct Process{}

impl ProcessCreate for Process {
    fn from_spec<P: ProcessSpec>(_spec: &P) -> Result<Process, Error> {
        Err(Error::from("platform not supported"))
    }
}
