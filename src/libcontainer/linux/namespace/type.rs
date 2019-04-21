use super::error::ErrorReason;
use crate::libcontainer::Error;

#[derive(Debug, PartialEq)]
pub enum NamespaceType {
    PID,
    UTS,
    IPC,
    USER,
    MOUNT,
    CGROUP,
    NETWORK,
}

impl NamespaceType {
    pub fn from_str(original: &str) -> Result<NamespaceType, Error> {
        match original {
            "pid" => Ok(NamespaceType::PID),
            "uts" => Ok(NamespaceType::UTS),
            "ipc" => Ok(NamespaceType::IPC),
            "user" => Ok(NamespaceType::USER),
            "mount" => Ok(NamespaceType::MOUNT),
            "cgroup" => Ok(NamespaceType::CGROUP),
            "network" => Ok(NamespaceType::NETWORK),
            _ => Err(Error::from(ErrorReason::InvalidNamespaceType)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_type_from_str() {
        let table = vec![
            ("pid", NamespaceType::PID),
            ("uts", NamespaceType::UTS),
            ("ipc", NamespaceType::IPC),
            ("user", NamespaceType::USER),
            ("mount", NamespaceType::MOUNT),
            ("cgroup", NamespaceType::CGROUP),
            ("network", NamespaceType::NETWORK),
        ];

        for (original, expect) in table {
            let result = NamespaceType::from_str(original);
            assert!(result.is_ok(), "expect {:?} to be ok", &result);
            assert_eq!(result.unwrap(), expect);
        }
    }

    #[test]
    fn namespace_type_from_str_returns_error_on_invalid_namespace() {
        let result = NamespaceType::from_str("invalid");
        assert!(result.is_err(), "expect {:?} to be ok", result);
    }
}


