extern crate serde;
extern crate nix;

#[cfg(target_os = "linux")]
pub mod linux;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IOError;

#[cfg(target_os = "linux")]
use std::error::Error;

pub fn read_config(config_path: &str) -> Result<BufReader<File>, IOError> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

#[cfg(target_os = "linux")]
pub fn build_paltform_spec<R: BufRead>(reader: R) -> Result<linux::spec::Spec, Box<Error>> {
    linux::spec::Spec::new(reader)
}

#[cfg(not(target_os = "linux"))]
pub fn build_paltform_spec<R: BufRead>(_reader: R) -> Result<(), &'static str> {
    Err("Platform not supported")
}

