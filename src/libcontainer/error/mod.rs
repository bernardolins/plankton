mod conv;

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Mount,
    Environment,
    Namespace,
    Config,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
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
