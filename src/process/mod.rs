pub mod platform;

use crate::Error;
use crate::spec::ProcessSpec;

pub trait ProcessCreate {
    fn from_spec<P: ProcessSpec>(spec: &P) -> Result<Self, Error> where Self: Sized;
}

pub trait ProcessRun {
    type PID;

    fn spawn(&self) -> Result<Self::PID, Error>;
    fn exec(&self) -> Result<(), Error>;
}

#[cfg(target_os = "linux")]
pub use self::platform::linux::Process as ContainerProcess;

#[cfg(not(target_os = "linux"))]
pub use self::platform::Process as ContainerProcess;
