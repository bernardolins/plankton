use crate::Error;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidNamespaceType,
    DuplicatedNamespace,
    InsufficientMemory,
    InvalidFlags,
    PermissionDenied,
    Unknown,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        let message = match kind {
            ErrorKind::InvalidNamespaceType => "invalid namespace type",
            ErrorKind::DuplicatedNamespace => "cannot set the same namespace twice",
            ErrorKind::InsufficientMemory => "insufficient memory (ENOMEM)",
            ErrorKind::InvalidFlags => "invalid flags when creating a namespace (EINVAL)",
            ErrorKind::PermissionDenied => "permission denied (EPERM)",
            ErrorKind::Unknown => "unknown error",
        };

        Error::from(message.to_string())
    }
}

impl From<nix::Error> for Error {
    fn from(nix_error: nix::Error) -> Error {
        match nix_error.as_errno() {
            Some(nix::errno::Errno::ENOMEM) => Error::from(ErrorKind::InsufficientMemory),
            Some(nix::errno::Errno::EINVAL) => Error::from(ErrorKind::InvalidFlags),
            Some(nix::errno::Errno::EPERM) => Error::from(ErrorKind::PermissionDenied),
            _ => Error::from(ErrorKind::Unknown),
        }
    }
}
