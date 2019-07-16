extern crate failure;

use std::fmt;
use std::fmt::Display;
use failure::Fail;
use failure::Context;
use failure::Backtrace;

pub struct Error {
    inner: Context<String>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error {
            inner: Context::new(msg.into()),
        }
    }
}

impl From<Context<String>> for Error {
    fn from(inner: Context<String>) -> Error {
        Error {
            inner,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(cause) = self.cause() {
            write!(f, "{}\nCause: {}", self.inner, cause)
        } else {
            write!(f, "{}", self.inner)
        }
    }
}
