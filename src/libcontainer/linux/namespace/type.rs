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
    fn from_str(original: &str) -> Option<NamespaceType> {
        match original {
            "pid" => Some(NamespaceType::PID),
            "uts" => Some(NamespaceType::UTS),
            "ipc" => Some(NamespaceType::IPC),
            "user" => Some(NamespaceType::USER),
            "mount" => Some(NamespaceType::MOUNT),
            "cgroup" => Some(NamespaceType::CGROUP),
            "network" => Some(NamespaceType::NETWORK),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_type_from_str() {
        let table = vec![
            ("pid", Some(NamespaceType::PID)),
            ("uts", Some(NamespaceType::UTS)),
            ("ipc", Some(NamespaceType::IPC)),
            ("user", Some(NamespaceType::USER)),
            ("mount", Some(NamespaceType::MOUNT)),
            ("cgroup", Some(NamespaceType::CGROUP)),
            ("network", Some(NamespaceType::NETWORK)),
            ("invalid", None),
        ];

        for (original, result) in table {
            assert_eq!(NamespaceType::from_str(original), result);
        }
    }
}


