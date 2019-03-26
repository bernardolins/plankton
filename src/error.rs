#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound,
    PermissionDenied,
    ConfigSyntax,
    ParseConfig,
    ConfigIO,
    Unknown,
    ContainerAlreadyExists,
    NoSuchFileOrDirectory,
    ResourceUnavailable,
    InvalidArgument,
    NotEnoughMemory,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            Error::NotFound => "no such file or directory",
            Error::PermissionDenied => "permission denied",
            Error::ConfigSyntax => "config.json is not a valid json",
            Error::ParseConfig => "invalid data on config.json",
            Error::ConfigIO => "error opening config.json",
            Error::Unknown => "unknown error",
            Error::ContainerAlreadyExists => "container already exists",
            Error::NoSuchFileOrDirectory => "no such file or directory",
            Error::ResourceUnavailable => "resource unavailable",
            Error::InvalidArgument => "invalid argument",
            Error::NotEnoughMemory => "not enough memory available",
        };
        write!(f, "{}", message)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Error {
        match io_error.kind() {
            std::io::ErrorKind::NotFound => Error::NotFound,
            std::io::ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::Unknown,
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::error::Error) -> Error {
        match serde_error.classify() {
            serde_json::error::Category::Io => Error::ConfigIO,
            serde_json::error::Category::Syntax => Error::ConfigSyntax,
            serde_json::error::Category::Data => Error::ParseConfig,
            serde_json::error::Category::Eof => Error::ParseConfig,
        }
    }
}

impl From<nix::Error> for Error {
    fn from(nix_error: nix::Error) -> Error {
        match nix_error.as_errno() {
            Some(nix::errno::Errno::EPERM) => Error::PermissionDenied,
            Some(nix::errno::Errno::ENOENT) => Error::NoSuchFileOrDirectory,
            Some(nix::errno::Errno::EAGAIN) => Error::ResourceUnavailable,
            Some(nix::errno::Errno::EINVAL) => Error::InvalidArgument,
            Some(nix::errno::Errno::ENOMEM) => Error::NotEnoughMemory,
            _ => Error::Unknown,
        }
    }
}
