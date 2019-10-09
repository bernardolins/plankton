use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use failure::ResultExt;
use std::path::PathBuf;
use std::collections::HashMap;

static TYPES: [&str; 7]  = [
    "pid",
    "uts",
    "ipc",
    "user",
    "mount",
    "cgroup",
    "network",
];

pub struct Namespaces {
    namespaces: HashMap<String, Option<PathBuf>>,
}

impl Namespaces {
    fn empty() -> Namespaces {
        Namespaces{ namespaces: HashMap::new() }
    }

    fn insert(&mut self, ns_type: String, path: Option<PathBuf>) -> Result<(), Error> {
        if !TYPES.contains(&ns_type.as_str()) {
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
        for t in TYPES.iter() {
            let result = namespaces.insert(t.to_string(), None);
            assert!(result.is_ok(), "expect {:?} to be ok", result);
        }
    }
}
