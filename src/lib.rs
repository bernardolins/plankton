extern crate serde;
extern crate nix;

pub mod spec;
pub mod container;
pub mod error;

use std::fs::File;
use std::io::BufReader;
use std::io::Error as IOError;


pub fn read_config(config_path: &str) -> Result<BufReader<File>, IOError> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
