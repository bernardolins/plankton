pub mod linux;

use crate::Error;
use crate::spec::Spec;
use super::ContainerCreate;

#[allow(dead_code)]
pub struct Container {}

impl ContainerCreate for Container {
    fn from_spec<S: Spec>(_spec: S) -> Result<Self, Error> {
        Err(Error::from("platform not supported"))
    }
}
