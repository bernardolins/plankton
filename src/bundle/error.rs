use crate::error::{Error, ErrorKind};

pub struct BundleError {
    pub path: String,
    pub message: String,
}

impl From<BundleError> for Error {
    fn from(bundle_error: BundleError) -> Error {
        Error::new(ErrorKind::Bundle, &format!("{}: {}", bundle_error.path, bundle_error.message))
    }
}
