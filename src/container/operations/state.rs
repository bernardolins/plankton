use crate::container::environment;
use crate::container::State;
use crate::error::Error;

pub fn run(container_id: &str) -> Result<String, Error> {
    let container = match environment::load(container_id) {
        Ok(Some(container)) => container,
        Ok(None) => {
            return Err(Error::NotFound)
        },
        Err(err) => {
            return Err(Error::from(err))
        }
    };

    let state = State::from(container);
    let json = serde_json::to_string_pretty(&state)?;

    Ok(json)
}
