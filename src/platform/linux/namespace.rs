use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use failure::ResultExt;
use nix::sched;
use nix::sched::CloneFlags;
use phf::phf_map;
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

static NS_KINDS: phf::Map<&'static str, CloneFlags> = phf_map! {
    "pid" => CloneFlags::CLONE_NEWPID,
    "uts" => CloneFlags::CLONE_NEWUTS,
    "ipc" => CloneFlags::CLONE_NEWIPC,
    "user" => CloneFlags::CLONE_NEWUSER,
    "mount" => CloneFlags::CLONE_NEWNS,
    "cgroup" => CloneFlags::CLONE_NEWCGROUP,
    "network" => CloneFlags::CLONE_NEWNET,
};

pub struct Namespaces {
    namespaces: HashMap<String, Option<PathBuf>>,
}

impl Namespaces {
    pub fn enter(&self) -> Result<(), Error> {
        let mut flags = CloneFlags::empty();
        for (kind, path) in &self.namespaces {
            let flag = NS_KINDS[kind.as_str()];
            if path.is_none() {
                flags.insert(flag);
            } else {
                let file = File::open(path.as_ref().unwrap()).context(format!("{:?}", &path))?;
                let fd = file.as_raw_fd();
                sched::setns(fd, flag).context(format!("{:?}: {:?}", &kind, &path))?;
            }
        }
        if !flags.is_empty() {
            sched::unshare(flags).context(format!("{:?}", flags))?;
        }
        Ok(())
    }

    fn empty() -> Namespaces {
        Namespaces{ namespaces: HashMap::new() }
    }

    fn insert(&mut self, ns_type: String, path: Option<PathBuf>) -> Result<(), Error> {
        if !NS_KINDS.contains_key(ns_type.as_str()) {
            Err(Error::from("invalid namespace type".to_string())).context(ns_type.clone())?;
        }
        if self.namespaces.contains_key(&ns_type) {
            Err(Error::from("duplicated namespace type".to_string())).context(ns_type.clone())?;
        }
        self.namespaces.insert(ns_type, path);
        Ok(())
    }
}

impl FromSpec<PosixSpec> for Namespaces {
    fn from_spec(spec: PosixSpec) -> Result<Namespaces, Error> {
        let mut namespaces = Namespaces::empty();
        for ns in spec.namespaces() {
            let ns_type = ns.ns_type().to_string();
            let ns_path = ns.path().map(|p| PathBuf::from(p));
            namespaces.insert(ns_type.to_string(), ns_path)?;
        }
        Ok(namespaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_empty_list() {
        let namespaces = Namespaces::empty();
        assert_eq!(namespaces.namespaces.len(), 0);
    }

    #[test]
    fn insert_returns_error_when_type_is_invalid() {
        let mut namespaces = Namespaces::empty();
        let result = namespaces.insert("invalid".to_string(), None);
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn insert_returns_error_with_duplicated_type() {
        let mut namespaces = Namespaces::empty();
        let result1 = namespaces.insert("pid".to_string(), None);
        let result2 = namespaces.insert("pid".to_string(), None);
        assert!(result1.is_ok(), "expect {:?} to be ok", result1);
        assert!(result2.is_err(), "expect {:?} to be err", result2);
    }

    #[test]
    fn insert_returns_ok_when_type_is_valid() {
        let mut namespaces = Namespaces::empty();
        for kind in NS_KINDS.keys() {
            let result = namespaces.insert(kind.to_string(), None);
            assert!(result.is_ok(), "expect {:?} to be ok", result);
        }
    }
}
