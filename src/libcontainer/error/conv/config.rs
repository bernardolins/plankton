use crate::libcontainer::Error;
use crate::libcontainer::error::ErrorKind;

use crate::libcontainer::config::Error as ConfigError;

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Error {
        Error {
            kind: ErrorKind::Config,
            message: format!("{}", err),
        }
    }
}
