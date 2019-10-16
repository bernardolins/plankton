mod posix;
pub use self::posix::PosixProcess;
mod env_vars;
pub use self::env_vars::EnvVars;
mod working_dir;
pub use self::working_dir::WorkingDir;

use crate::Error;
use std::process::Child;

pub trait Process {
    fn before_exec<F>(&mut self, func: F) where F: FnMut() -> Result<(), Error> + Send + Sync + 'static;
    fn spawn(&mut self) -> Result<Child, Error>;
}
