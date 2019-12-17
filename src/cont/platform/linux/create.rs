use crate::Error;
use crate::spec::Spec;
use crate::cont::ContainerCreate;
use super::*;

impl ContainerCreate for Container {
    fn from_spec<S: Spec>(spec: S) -> Result<Self, Error> {
        let mut container = Container{
            init_process: None,
            namespaces: None,
        };

        container.init_process(spec.get_process())?;
        container.linux(spec.get_linux())?;

        Ok(container)
    }
}
