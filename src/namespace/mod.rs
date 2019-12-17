pub mod set;
pub use self::set::NamespaceSet;

use crate::Error;
use failure::ResultExt;
use nix::sched::CloneFlags;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Namespace {
    CGROUP,
    IPC,
    MOUNT,
    NETWORK,
    PID,
    USER,
    UTS,
}

impl TryFrom<&str> for Namespace {
    type Error = Error;

    fn try_from(str_type: &str) -> Result<Namespace, Error> {
        match str_type {
            "cgroup" => Ok(Namespace::CGROUP),
            "ipc" => Ok(Namespace::IPC),
            "mount" => Ok(Namespace::MOUNT),
            "network" => Ok(Namespace::NETWORK),
            "pid" => Ok(Namespace::PID),
            "user" => Ok(Namespace::USER),
            "uts" => Ok(Namespace::UTS),
            _ => {
                Err(
                    Error::from("invalid namespace type")
                ).context(str_type.to_string())?
            }
        }
    }
}

impl Namespace {
    fn to_flag(&self) -> CloneFlags {
        match self {
            Namespace::CGROUP => CloneFlags::CLONE_NEWCGROUP,
            Namespace::IPC => CloneFlags::CLONE_NEWIPC,
            Namespace::MOUNT => CloneFlags::CLONE_NEWNS,
            Namespace::NETWORK => CloneFlags::CLONE_NEWNET,
            Namespace::PID => CloneFlags::CLONE_NEWPID,
            Namespace::USER => CloneFlags::CLONE_NEWUSER,
            Namespace::UTS => CloneFlags::CLONE_NEWUTS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_with_valid_value() {
        let table = vec![
            ("cgroup", Namespace::CGROUP),
            ("ipc", Namespace::IPC),
            ("mount", Namespace::MOUNT),
            ("network", Namespace::NETWORK),
            ("pid", Namespace::PID),
            ("user", Namespace::USER),
            ("uts", Namespace::UTS),
        ];
        for (original, expect) in table {
            let result = Namespace::try_from(original);
            assert!(result.is_ok(), "expect {:?} to be ok", &result);
            assert_eq!(result.unwrap(), expect);
        }
    }

    #[test]
    fn try_from_str_with_invalid_value() {
        let result = Namespace::try_from("invalid");
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn to_flag_with_valid_value() {
        let table = vec![
            (Namespace::CGROUP, CloneFlags::CLONE_NEWCGROUP),
            (Namespace::IPC, CloneFlags::CLONE_NEWIPC),
            (Namespace::MOUNT, CloneFlags::CLONE_NEWNS),
            (Namespace::NETWORK, CloneFlags::CLONE_NEWNET),
            (Namespace::PID, CloneFlags::CLONE_NEWPID),
            (Namespace::USER, CloneFlags::CLONE_NEWUSER),
            (Namespace::UTS, CloneFlags::CLONE_NEWUTS),
        ];
        for (original, expect) in table {
            assert_eq!(original.to_flag(), expect);
        }
    }
}
