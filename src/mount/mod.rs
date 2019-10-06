mod posix;

use crate::Error;

pub trait Mounts {
    fn mount_all(&self) -> Result<(), Error>;
}
