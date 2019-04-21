use crate::libcontainer::Error;
use crate::libcontainer::error::ErrorKind;

use crate::libcontainer::linux::namespace::ErrorReason as NamespaceError;

impl From<NamespaceError> for Error {
    fn from(err: NamespaceError) -> Error {
        Error {
            kind: ErrorKind::Namespace,
            message: format!("{}", err),
        }
    }
}
