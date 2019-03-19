#[cfg(target_os = "linux")]
pub mod linux;

use std::io::BufRead;
use std::error::Error;

#[cfg(target_os = "linux")]
pub fn build<R: BufRead>(reader: R) -> Result<linux::Spec, Box<Error>> {
    linux::Spec::new(reader)
}

#[cfg(not(target_os = "linux"))]
pub fn build<R: BufRead>(_reader: R) -> Result<(), &'static str> {
    Err("Platform not supported")
}
