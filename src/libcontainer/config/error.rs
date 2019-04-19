#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "config.json: {}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Error {
        Error {
            message: format!("{}", io_error),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::error::Error) -> Error {
        Error {
            message: format!("{}", serde_error),
        }
    }
}
