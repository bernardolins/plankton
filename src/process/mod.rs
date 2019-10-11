mod posix;
pub use self::posix::PosixProcess;

mod env_vars;
pub use self::env_vars::EnvVars;

mod working_dir;
pub use self::working_dir::WorkingDir;

use crate::Error;

trait Process {
    type ProcessID;

    fn spawn(&self) -> Result<Self::ProcessID, Error>;
}
