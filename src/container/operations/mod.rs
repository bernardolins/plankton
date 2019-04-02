mod run;
mod state;

use crate::error::Error;

pub fn run(id: &str, bundle_path: &str) -> Result<(), Error> {
    run::run(id, bundle_path)
}

pub fn state(container_id: &str) -> Result<String, Error> {
    state::run(container_id)
}
