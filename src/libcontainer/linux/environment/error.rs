use crate::Error;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    PrivateHostname,
    InvalidWorkingDir,
    WrongEnvVarFormat,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        let message = match kind {
            ErrorKind::PrivateHostname => "container needs a private UTS namespace in order to set hostname",
            ErrorKind::InvalidWorkingDir => "working dir is not a valid absolute path",
            ErrorKind::WrongEnvVarFormat => "environment variable must have 'KEY=VALUE' format",
        };

        Error::from(message.to_string())
    }
}
