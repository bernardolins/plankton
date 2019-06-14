#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Bundle,
}

pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            message: String::from(message),
            kind: kind,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
