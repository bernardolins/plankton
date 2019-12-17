use crate::Error;
use crate::spec::NamespaceSpec;
use failure::ResultExt;
use std::collections::HashMap;
use std::path::PathBuf;
use std::convert::TryFrom;
use super::Namespace;

#[derive(Debug, PartialEq, Clone)]
pub struct NamespaceSet {
    to_create: Vec<Namespace>,
    to_enter: HashMap<Namespace, PathBuf>,
}

impl NamespaceSet {
    pub fn from_spec<N: NamespaceSpec>(ns_spec: Option<&Vec<N>>) -> Result<NamespaceSet, Error> {
        if ns_spec.is_none() {
            return Ok(NamespaceSet{to_create: Vec::new(), to_enter: HashMap::new()});
        }
        let ns_list = ns_spec.unwrap();
        let mut to_create: Vec<Namespace> = Vec::new();
        let mut to_enter: HashMap<Namespace, PathBuf> = HashMap::new();
        for spec_ns in ns_list.iter() {
            let t = spec_ns.get_type_clone();
            let p = spec_ns.get_path_clone();
            let ns = Namespace::try_from(t.as_ref())?;
            if to_create.contains(&ns) || to_enter.contains_key(&ns) {
                Err(Error::from("duplicated namespace type")).context(t)?;
            }
            if p.is_none() {
                to_create.push(ns);
            } else {
                to_enter.insert(ns, p.unwrap());
            }
        }
        Ok(NamespaceSet{to_create, to_enter})
    }

    pub fn contains(&self, ns: &Namespace) -> bool {
        self.to_create.contains(ns) || self.to_enter.contains_key(ns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct FakeNamespace{
        fake_type: String,
        fake_path: Option<PathBuf>,
    }

    impl NamespaceSpec for FakeNamespace {
        fn get_type(&self) -> &String { &self.fake_type }
        fn get_type_clone(&self) -> String { self.fake_type.clone() }
        fn get_path(&self) -> Option<&PathBuf> { self.fake_path.as_ref() }
        fn get_path_clone(&self) -> Option<PathBuf> { self.get_path().cloned() }
    }

    #[test]
    fn from_spec_with_none_paths() {
        let namespaces = Some(vec![
            FakeNamespace{fake_type: "cgroup".to_string(), fake_path: None},
            FakeNamespace{fake_type: "ipc".to_string(), fake_path: None},
        ]);
        let result = NamespaceSet::from_spec(namespaces.as_ref());
        let expected_to_create = vec![Namespace::CGROUP, Namespace::IPC];
        let expected_to_enter = HashMap::<Namespace, PathBuf>::new();
        assert!(result.is_ok(), "expected {:?} to be err", result);
        let ns_set = result.unwrap();
        assert_eq!(&ns_set.to_create, &expected_to_create);
        assert_eq!(&ns_set.to_enter, &expected_to_enter);
    }

    #[test]
    fn from_spec_with_some_paths() {
        let namespaces = Some(vec![
            FakeNamespace{fake_type: "pid".to_string(), fake_path: None},
            FakeNamespace{fake_type: "user".to_string(), fake_path: Some(PathBuf::from("/proc/1234/ns/user"))},
            FakeNamespace{fake_type: "mount".to_string(), fake_path: None},
        ]);
        let result = NamespaceSet::from_spec(namespaces.as_ref());
        let expected_to_create = vec![
            Namespace::PID,
            Namespace::MOUNT,
        ];
        let expected_to_enter: HashMap<Namespace, PathBuf> = [
            (Namespace::USER, PathBuf::from("/proc/1234/ns/user"))
        ].iter().cloned().collect();
        assert!(result.is_ok(), "expected {:?} to be err", result);
        let ns_set = result.unwrap();
        assert_eq!(&ns_set.to_create, &expected_to_create);
        assert_eq!(&ns_set.to_enter, &expected_to_enter);
    }

    #[test]
    fn from_spec_with_invalid_type() {
        let namespaces = Some(vec![
            FakeNamespace{fake_type: "invalid".to_string(), fake_path: None},
            FakeNamespace{fake_type: "user".to_string(), fake_path: Some(PathBuf::from("/proc/1234/ns/user"))},
            FakeNamespace{fake_type: "mount".to_string(), fake_path: None},
        ]);
        let result = NamespaceSet::from_spec(namespaces.as_ref());
        assert!(result.is_err(), "expected {:?} to be err", result);
    }

    #[test]
    fn from_spec_with_duplicated_types() {
        let namespaces = Some(vec![
            FakeNamespace{fake_type: "pid".to_string(), fake_path: None},
            FakeNamespace{fake_type: "pid".to_string(), fake_path: Some(PathBuf::from("/proc/1234/ns/pid"))},
        ]);
        let result = NamespaceSet::from_spec(namespaces.as_ref());
        assert!(result.is_err(), "expected {:?} to be err", result);
    }

    #[test]
    fn from_spec_with_duplicated_types_and_no_path() {
        let namespaces = Some(vec![
            FakeNamespace{fake_type: "pid".to_string(), fake_path: None},
            FakeNamespace{fake_type: "pid".to_string(), fake_path: None},
        ]);
        let result = NamespaceSet::from_spec(namespaces.as_ref());
        assert!(result.is_err(), "expected {:?} to be err", result);
    }

    #[test]
    fn contains() {
        let namespaces = Some(vec![
            FakeNamespace{fake_type: "pid".to_string(), fake_path: None},
        ]);
        let set = NamespaceSet::from_spec(namespaces.as_ref()).unwrap();
        assert!(set.contains(&Namespace::PID));
        assert!(!set.contains(&Namespace::UTS));
    }
}
