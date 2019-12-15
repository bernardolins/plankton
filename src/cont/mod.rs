#[path = "linux.rs"]
#[cfg(unix)]
mod imp;

use crate::Error;
use crate::spec::Spec;

#[derive(Debug, PartialEq)]
pub struct Container {
    inner: imp::Container,
}

impl Container {
    pub fn from_spec<S: Spec>(spec: S) -> Result<Container, Error> {
        Ok(Container{
            inner: imp::Container::from_spec(spec)?,
        })
    }
}
