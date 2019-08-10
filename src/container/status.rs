use std::fmt;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match *self {
            Status::Creating => "creating",
            Status::Created => "created",
            Status::Running => "running",
            Status::Stopped => "stopped",
        };

        write!(f, "{}", status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_implements_debug_trait() {
        let table = vec![
            (Status::Creating, "creating"),
            (Status::Created, "created"),
            (Status::Running, "running"),
            (Status::Stopped, "stopped"),
        ];

        for (original, expect) in table {
            let result = format!("{:?}", original);
            assert_eq!(result, expect);
        }

    }
}
