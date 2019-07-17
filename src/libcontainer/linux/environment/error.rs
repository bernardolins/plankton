use crate::libcontainer::Error;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    WorkingDir,
    Hostname,
    EnvVar,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        let message = match kind {
            ErrorKind::EnvVar => "wrong environment variable format",
            ErrorKind::WorkingDir => "container working dir must be a valid absolute path",
            ErrorKind::Hostname => "container needs a private UTS namespace in order to set hostname",
        };

        Error::from(message.to_string())
    }
}
