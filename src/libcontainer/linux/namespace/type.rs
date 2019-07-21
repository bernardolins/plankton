use super::error::ErrorKind;
use crate::Error;
use serde::Deserialize;
use serde::Serialize;
use failure::ResultExt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
            _ => Err(Error::from(ErrorKind::InvalidNamespaceType)).context(original.to_string())?,
        }
    }

    pub fn to_string(&self) -> String {
        let str_type = match *self {
            NamespaceType::PID => "pid",
            NamespaceType::UTS => "uts",
            NamespaceType::IPC => "ipc",
            NamespaceType::USER => "user",
            NamespaceType::MOUNT => "mount",
            NamespaceType::CGROUP => "cgroup",
            NamespaceType::NETWORK => "network",
        };
        str_type.to_string()
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

    #[test]
    fn namespace_type_to_string() {
        let table = vec![
            (NamespaceType::PID, "pid"),
            (NamespaceType::UTS, "uts"),
            (NamespaceType::IPC, "ipc"),
            (NamespaceType::USER, "user"),
            (NamespaceType::MOUNT, "mount"),
            (NamespaceType::CGROUP, "cgroup"),
            (NamespaceType::NETWORK, "network"),
        ];

        for (original, expect) in table {
            let result = original.to_string();
            assert_eq!(result, expect, "expect {} to be {}", expect, result);
        }
    }
}


