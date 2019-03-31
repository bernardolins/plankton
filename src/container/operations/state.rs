use crate::container::Container;
use crate::container::State;
use crate::error::Error;

pub fn run(container_id: &str) -> Result<String, Error> {
    let container = Container::load(container_id)?;
    let state = State::from(container);
    let json = serde_json::to_string_pretty(&state)?;

    Ok(json)
}
