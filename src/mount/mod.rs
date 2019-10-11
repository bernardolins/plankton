mod posix;
pub use self::posix::PosixMounts;

use crate::Error;

pub trait Mounts {
    fn mount_all(&self) -> Result<(), Error>;
}
