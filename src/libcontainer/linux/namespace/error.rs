#[derive(Debug, PartialEq)]
pub enum ErrorReason {
    InvalidNamespaceType,
    DuplicatedNamespace,
    InsufficientMemory,
    InvalidFlags,
    PermissionDenied,
    Unknown,
}

impl std::fmt::Display for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            ErrorReason::InvalidNamespaceType => "invalid namespace type",
            ErrorReason::DuplicatedNamespace => "cannot set the same namespace twice",
            ErrorReason::InsufficientMemory => "insufficient memory (ENOMEM)",
            ErrorReason::InvalidFlags => "invalid flags when creating a namespace (EINVAL)",
            ErrorReason::PermissionDenied => "permission denied (EPERM)",
            ErrorReason::Unknown => "unknown error",
        };
        write!(f, "{}", message)
    }
}

impl From<nix::Error> for ErrorReason {
    fn from(nix_error: nix::Error) -> ErrorReason {
        match nix_error.as_errno() {
            Some(nix::errno::Errno::ENOMEM) => ErrorReason::InsufficientMemory,
            Some(nix::errno::Errno::EINVAL) => ErrorReason::InvalidFlags,
            Some(nix::errno::Errno::EPERM) => ErrorReason::PermissionDenied,
            _ => ErrorReason::Unknown,
        }
    }
}
