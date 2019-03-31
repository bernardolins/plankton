use crate::container::Container;
use serde::Serialize;

#[derive(Serialize)]
pub struct State {
    oci_version: String,
    id: String,
    pid: Option<i32>,
    status: String,
    bundle: String,
}

impl From<Container> for State {
    fn from(container: Container) -> State {
        State {
            oci_version: String::from(container.config.oci_version()),
            id: String::from(container.id),
            pid: container.pid,
            status: String::from(container.status.to_str()),
            bundle: container.bundle_path,
        }
    }
}
