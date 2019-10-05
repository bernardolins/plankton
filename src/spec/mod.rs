pub mod posix;
pub use self::posix::PosixSpec;

use crate::Error;
use std::io::Read;

pub trait Spec {
    type RootSpec;
    type MountSpec;
    type ProcessSpec;

    fn from_reader<R: Read>(reader: R) -> Result<Self, Error> where Self: Sized;

    fn oci_version(&self) -> &str;
    fn hostname(&self) -> &Option<String>;
    fn root(&self) -> &Self::RootSpec;
    fn mounts(&self) -> &Vec<Self::MountSpec>;
    fn process(&self) -> &Self::ProcessSpec;
}

pub trait FromSpec<S: Spec> {
    type Error;

    fn from_spec(spec: S) -> Result<Self, Error> where Self: Sized;
}
