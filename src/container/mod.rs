pub mod status;
pub use self::status::Status;

pub mod state;
pub use self::state::State;

use crate::Error;

pub trait Operations {
    fn create(container_id: &str, bundle_dir: &str) -> Result<(), Error>;
}
