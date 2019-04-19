use crate::libcontainer::linux::mount::Error as MountError;
use crate::libcontainer::linux::namespace::Error as NamespaceError;
use crate::libcontainer::linux::environment::Error as EnvironmentError;
use crate::libcontainer::config::Error as ConfigError;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {
    MOUNT,
    ENVIRONMENT,
    NAMESPACE,
    CONFIG,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            ErrorKind::MOUNT => "mount error",
            ErrorKind::ENVIRONMENT => "environment error",
            ErrorKind::NAMESPACE => "namespace error",
            ErrorKind::CONFIG => "config error",

        };
        write!(f, "{}", message)
    }
}

impl From<MountError> for Error {
    fn from(err: MountError) -> Error {
        Error {
            kind: ErrorKind::MOUNT,
            message: format!("{}", err),
        }
    }
}

impl From<NamespaceError> for Error {
    fn from(err: NamespaceError) -> Error {
        Error {
            kind: ErrorKind::NAMESPACE,
            message: format!("{}", err),
        }
    }
}

impl From<EnvironmentError> for Error {
    fn from(err: EnvironmentError) -> Error {
        Error {
            kind: ErrorKind::ENVIRONMENT,
            message: format!("{}", err),
        }
    }
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Error {
        Error {
            kind: ErrorKind::CONFIG,
            message: format!("{}", err),
        }
    }
}
