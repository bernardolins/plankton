#[cfg(target_os = "linux")]
mod run;

#[cfg(target_os = "linux")]
mod state;

use crate::error::Error;

#[cfg(target_os = "linux")]
pub fn run(id: &str, bundle_path: &str) -> Result<(), Error> {
    run::run(id, bundle_path)
}

#[cfg(not(target_os = "linux"))]
pub fn run(id: &str, bundle_path: &str) -> Result<(), Error> {
    Err(Error::PlatformNotSupported)
}

#[cfg(target_os = "linux")]
pub fn state(container_id: &str) -> Result<String, Error> {
    state::run(container_id)
}

#[cfg(not(target_os = "linux"))]
pub fn state(container_id: &str) -> Result<String, Error> {
    Err(Error::PlatformNotSupported)
}
