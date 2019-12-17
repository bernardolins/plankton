mod platform;

use crate::Error;
use crate::spec::Spec;

pub trait ContainerCreate {
    fn from_spec<S: Spec>(spec: S) -> Result<Self, Error> where Self: Sized;
}

#[cfg(target_os = "linux")]
pub use self::platform::linux::Container;

#[cfg(not(target_os = "linux"))]
pub use self::platform::Container;
