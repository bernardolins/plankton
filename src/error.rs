extern crate failure;

use failure::Fail;
use failure::Context;
use failure::Backtrace;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)?;

        if let Some(cause) = self.cause() {
            write!(f, "\nReason: {}", cause)?;
        }

        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
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

impl From<Error> for IOError {
    fn from(error: Error) -> IOError {
        eprintln!("{}", error);
        IOError::new(IOErrorKind::Other, format!("{:?}", error))
    }
}
