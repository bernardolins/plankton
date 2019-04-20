use crate::libcontainer::Error;
use crate::libcontainer::error::ErrorKind;

use crate::libcontainer::linux::mount::Error as MountError;

impl From<MountError> for Error {
    fn from(err: MountError) -> Error {
        Error {
            kind: ErrorKind::Mount,
            message: format!("{}", err),
        }
    }
}
