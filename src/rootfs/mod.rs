mod linux;
pub use self::linux::LinuxRootFS;

use crate::Error;

pub trait RootFS {
    fn set(&self) -> Result<(), Error>;
}
