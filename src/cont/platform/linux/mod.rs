pub mod create;

use crate::Error;
use crate::namespace::NamespaceSet;
use crate::process::ContainerProcess;
use crate::spec::LinuxSpec;
use crate::spec::ProcessSpec;
use crate::process::ProcessCreate;

#[derive(Debug, PartialEq)]
pub struct Container {
    init_process: Option<ContainerProcess>,
    namespaces: Option<NamespaceSet>,
}

impl Container {
    fn init_process<P: ProcessSpec>(&mut self, spec: Option<&P>) -> Result<(), Error> {
        if spec.is_none() {
            self.init_process = None;
            return Ok(())
        }
        let ip = ContainerProcess::from_spec(spec.unwrap())?;
        self.init_process = Some(ip);
        Ok(())
    }

    fn linux<L: LinuxSpec>(&mut self, spec: Option<&L>) -> Result<(), Error> {
        if spec.is_none() {
            self.namespaces = None;
            return Ok(())
        }
        let linux_spec = spec.unwrap();
        self.namespaces = Some(NamespaceSet::from_spec(linux_spec.get_namespaces())?);
        Ok(())
    }
}
