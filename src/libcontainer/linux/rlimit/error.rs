#[derive(Debug, PartialEq)]
pub struct ErrorReason {
    message: String,
}

impl ErrorReason {
    pub fn new(message: &str) -> ErrorReason {
        ErrorReason {
            message: String::from(message),
        }
    }
}

impl std::fmt::Display for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rlimit error: {}", &self.message)
    }
}
