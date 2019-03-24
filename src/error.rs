#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound,
    PermissionDenied,
    ConfigSyntax,
    ParseConfig,
    ConfigIO,
    Unknown,
    ContainerAlreadyExists,
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
