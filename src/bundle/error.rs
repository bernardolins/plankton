use crate::error;

pub struct Error {
    pub path: String,
    pub message: String,
}

impl From<Error> for error::Error {
    fn from(bundle_error: Error) -> error::Error {
        error::Error::new(
            error::ErrorKind::Bundle,
            &format!("{}: {}", bundle_error.path, bundle_error.message)
        )
    }
}
