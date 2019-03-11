use std;
use std::fmt;

#[derive(Debug)]
pub struct ConfigError {
    reason: String,
}

impl ConfigError {
    pub fn new(reason: &str) -> ConfigError {
        ConfigError { reason: reason.to_string() }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Config error: {}", self.reason)
    }
}

impl std::error::Error for ConfigError {
    fn description(&self) -> &str {
        &self.reason
    }
}
