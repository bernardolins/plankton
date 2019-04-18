#[derive(Debug, PartialEq)]
pub enum Error {
    WorkingDir,
    Hostname,
    EnvVar,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            Error::WorkingDir => "container working dir must be a valid absolute path",
            Error::Hostname => "container needs a private UTS namespace in order to set hostname",
            Error::EnvVar => "wrong environment variable format",
        };
        write!(f, "{}", message)
    }
}
