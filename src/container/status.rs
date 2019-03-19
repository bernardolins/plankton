#[derive(Debug, PartialEq)]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match self {
            Status::Creating => "creating",
            Status::Created => "created",
            Status::Running => "running",
            Status::Stopped => "stopped",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str() {
        let table = vec![
            (Status::Creating, "creating"),
            (Status::Created, "created"),
            (Status::Running, "running"),
            (Status::Stopped, "stopped"),
        ];

        for (status, expected) in table {
            assert_eq!(status.to_str(), expected);
        }
    }
}
