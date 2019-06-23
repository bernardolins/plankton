use std::io;
use std::path::PathBuf;

pub struct Error {
    message: String,
}

pub enum ErrorKind {
    Filesystem(PathBuf)
}

impl From<ErrorKind> for Error {
    fn from(error_kind: ErrorKind) -> Error {
        let message = match error_kind {
            ErrorKind::Filesystem(pathbuf) => format!("{}: {}", pathbuf.to_str().unwrap(), io::Error::last_os_error()),
        };

        Error { message }
    }
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: String::from(message),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
