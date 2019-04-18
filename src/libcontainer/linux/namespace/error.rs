#[derive(Debug, PartialEq)]
pub enum Error {
    DuplicatedNamespace,
    InsufficientMemory,
    InvalidFlags,
    PermissionDenied,
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            Error::DuplicatedNamespace => "cannot set the same namespace twice",
            Error::InsufficientMemory => "insufficient memory (ENOMEM)",
            Error::InvalidFlags => "invalid flags when creating a namespace (EINVAL)",
            Error::PermissionDenied => "permission denied (EPERM)",
            Error::Unknown => "unknown error",
        };
        write!(f, "{}", message)
    }
}

impl From<nix::Error> for Error {
    fn from(nix_error: nix::Error) -> Error {
        match nix_error.as_errno() {
            Some(nix::errno::Errno::ENOMEM) => Error::InsufficientMemory,
            Some(nix::errno::Errno::EINVAL) => Error::InvalidFlags,
            Some(nix::errno::Errno::EPERM) => Error::PermissionDenied,
            _ => Error::Unknown,
        }
    }
}
