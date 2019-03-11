#[cfg(target_os = "linux")]
use std::error::Error;

#[cfg(target_os = "linux")]
pub mod linux;

pub mod error;

#[cfg(target_os = "linux")]
pub fn load_spec(path: &str) -> Result<linux::Spec, Box<Error>> {
    linux::Spec::from_json(path)
}


#[cfg(not(target_os = "linux"))]
pub fn load_spec(_path: &str) -> Result<(), &'static str> {
    Err("Platform not supported")
}
