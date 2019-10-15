mod linux;
pub use self::linux::LinuxRootFS;

use crate::Error;

pub trait RootFS {
    fn set(&self, bundle_dir: &str) -> Result<(), Error>;
}
