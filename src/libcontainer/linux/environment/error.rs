#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    WorkingDir,
    Hostname,
    EnvVar,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            ErrorKind::WorkingDir => "container working dir must be a valid absolute path",
            ErrorKind::Hostname => "container needs a private UTS namespace in order to set hostname",
            ErrorKind::EnvVar => "wrong environment variable format",
        };
        write!(f, "{}", message)
    }
}
