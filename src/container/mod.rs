mod status;

use std::error::Error;
use self::status::Status;

struct Container {
    id: String,
    status: Status,
}

impl Container {
    pub fn new(id: &str) -> Result<Container, Box<Error>> {
        let container = Container {status: Status::Creating};
            id: id.to_string(),
        Ok(container)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let container = Container::new("test_container").unwrap();
        assert_eq!(container.id, "test_container");
        assert_eq!(container.status, Status::Creating);
    }
}
