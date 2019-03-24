use crate::container::Container;

#[derive(Debug)]
pub struct State {
    oci_version: String,
    id: String,
    status: String,
}

impl From<&Container> for State {
    fn from(container: &Container) -> State {
        State {
            oci_version: String::from(container.oci_version()),
            id: String::from(container.id()),
            status: String::from(container.status().to_str()),
        }
    }
}
