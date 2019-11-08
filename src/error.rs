extern crate failure;

use std::fmt;
use std::fmt::Debug;
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
        write!(f, "{}", self.inner)?;

        if let Some(cause) = self.cause() {
            write!(f, "\nReason: {}", cause)?;
        }

        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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


impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Error {
        let msg = String::from(msg);
        Error {
            inner: Context::new(msg.into()),
        }
    }
}

impl From<Context<&'static str>> for Error {
    fn from(inner: Context<&'static str>) -> Error {
        let inner = Context::new(inner.get_context().to_string());
        Error {
            inner,
        }
    }
}
