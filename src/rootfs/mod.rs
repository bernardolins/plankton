mod linux;

use crate::Error;

pub trait RootFS {
    fn set(&self) -> Result<(), Error>;
}
