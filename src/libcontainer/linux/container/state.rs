use super::Container;
use crate::container::State;

impl From<Container> for State {
    fn from(container: Container) -> State {
        State {
            id: String::from(container.id),
            pid: container.init_pid,
            status: String::from(container.status.to_string()),
            bundle: String::from("/"),
            oci_version: String::from("1.0.0"),
        }
    }
}
