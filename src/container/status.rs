use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

impl Status {
    pub fn to_string(&self) -> String {
        let status = match *self {
            Status::Creating => "creating",
            Status::Created => "created",
            Status::Running => "running",
            Status::Stopped => "stopped",
        };

        status.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_str_returns_the_str_version_of_status() {
        let table = vec![
            (Status::Creating, "creating"),
            (Status::Created, "created"),
            (Status::Running, "running"),
            (Status::Stopped, "stopped"),
        ];

        for (original, expect) in table {
            let result = original.to_string();
            assert_eq!(result, expect);
        }

    }
}
