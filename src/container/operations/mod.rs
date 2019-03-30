mod create;
mod state;

use crate::container::Container;
use crate::error::Error;

pub fn create(id: &str, bundle_path: &str) -> Result<Container, Error> {
    create::run(id, bundle_path)
}

pub fn state(container_id: &str) -> Result<String, Error> {
    state::run(container_id)
}
