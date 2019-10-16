mod linux;
pub use self::linux::LinuxRootFS;

use crate::Error;
use std::path::PathBuf;

pub trait RootFS {
    fn set(&self, bundle_path: PathBuf) -> Result<(), Error>;
}
