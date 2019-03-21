use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: String,
    reason: String,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Container Error: {} - {}", self.kind, self.reason)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            kind: String::from("I/O Error"),
            reason: error.to_string()
        }
    }
}
