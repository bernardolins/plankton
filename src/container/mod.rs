mod status;

use std::error::Error;
use self::status::Status;

struct Container {
    status: Status,
}

impl Container {
    fn new() -> Result<Container, Box<Error>> {
        let container = Container {status: Status::Creating};
        Ok(container)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let container = Container::new().unwrap();
        assert_eq!(container.status, Status::Creating)
    }
}
