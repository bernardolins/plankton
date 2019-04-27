use crate::libcontainer::Error;
use crate::libcontainer::error::ErrorKind;

use crate::libcontainer::linux::rlimit::ErrorKind as RlimitError;

impl From<RlimitError> for Error {
    fn from(err: RlimitError) -> Error {
        Error {
            kind: ErrorKind::Rlimit,
            message: format!("{}", err),
        }
    }
}
