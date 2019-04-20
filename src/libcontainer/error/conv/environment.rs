use crate::libcontainer::Error;
use crate::libcontainer::error::ErrorKind;

use crate::libcontainer::linux::environment::ErrorKind as EnvironmentError;

impl From<EnvironmentError> for Error {
    fn from(err: EnvironmentError) -> Error {
        Error {
            kind: ErrorKind::Environment,
            message: format!("{}", err),
        }
    }
}
