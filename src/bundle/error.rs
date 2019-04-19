#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bundle error: {}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Error {
        Error {
            message: format!("{}", io_error),
        }
    }
}
