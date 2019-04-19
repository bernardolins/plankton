pub mod error;
pub mod conv;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::libcontainer::linux::namespace::NamespaceType;
use crate::libcontainer::linux::namespace::NamespaceList;

pub use self::error::Error;

const DEFAULT_WORKING_DIR: &str = "/";

#[derive(Debug)]
pub struct Environment {
    argv: Vec<String>,
    rootfs: PathBuf,
    working_dir: PathBuf,
    hostname: Option<String>,
    namespaces: NamespaceList,
}

impl Environment {
    pub fn new(argv: &[String], rootfs: &str) -> Environment {
        Environment {
            argv: argv.to_vec(),
            rootfs: PathBuf::from(rootfs),
            working_dir: PathBuf::from(DEFAULT_WORKING_DIR),
            hostname: None,
            namespaces: NamespaceList::empty(),
        }
    }

    pub fn argv(&self) -> &Vec<String> {
        &self.argv
    }

    pub fn rootfs(&self) -> &PathBuf {
        &self.rootfs
    }

    pub fn working_dir(&self) -> &PathBuf {
        &self.working_dir
    }

    pub fn hostname(&self) -> &Option<String> {
        &self.hostname
    }

    pub fn namespaces(&self) -> &NamespaceList {
        &self.namespaces
    }

    pub fn set_working_dir(&mut self, working_dir: &str) -> Result<(), Error> {
        let cwd = PathBuf::from(working_dir);

        if cwd.is_absolute() {
            self.working_dir = cwd;
            Ok(())
        } else {
            Err(Error::WorkingDir)
        }
    }

    pub fn set_hostname(&mut self, hostname: &str) -> Result<(), Error> {
        if self.namespaces.contains_type(&NamespaceType::UTS) {
            self.hostname = Some(String::from(hostname));
            Ok(())
        } else {
            Err(Error::Hostname)
        }
    }

    pub fn set_namespaces(&mut self, ns_list: NamespaceList) {
        self.namespaces = ns_list;
    }


    pub fn set_env_var(&self, k: &str, v: &str) -> Result<(), Error> {
        let key = OsStr::new(k);
        let val = OsStr::new(v);

        if key.is_empty() {
            return Err(Error::EnvVar);
        } else {
            env::set_var(key, val);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::libcontainer::linux::namespace::Namespace;
    use crate::libcontainer::linux::namespace::NamespaceList;
    use crate::libcontainer::linux::namespace::NamespaceType;

    #[test]
    fn environment_new() {
        let environment = Environment::new(&["sh"], "rootfs");

        assert_eq!(environment.argv, vec!["sh"]);
        assert_eq!(environment.rootfs, PathBuf::from("rootfs"));
    }

    #[test]
    fn environment_argv() {
        let environment = Environment::new(&["sh"], "rootfs");

        assert_eq!(environment.argv(), &["sh"]);
    }

    #[test]
    fn environment_working_dir_defaults_to_root() {
        let environment = Environment::new(&["sh"], "rootfs");

        assert_eq!(environment.working_dir(), &PathBuf::from("/"));
    }

    #[test]
    fn environment_namespaces_defaults_to_empty_list() {
        let environment = Environment::new(&["sh"], "rootfs");

        assert_eq!(environment.namespaces().as_vec().len(), 0);
    }

    #[test]
    fn environment_set_working_dir() {
        let mut environment = Environment::new(&["sh"], "rootfs");
        let result = environment.set_working_dir("/tmp");

        assert!(result.is_ok(), "expect {:?} to be ok", &result);
        assert_eq!(environment.working_dir(), &PathBuf::from("/tmp"));
    }

    #[test]
    fn environment_set_working_dir_relative_path() {
        let mut environment = Environment::new(&["sh"], "rootfs");
        let result = environment.set_working_dir("./");

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Error::WorkingDir);
    }

    #[test]
    fn environment_hostname_defaults_to_none() {
        let environment = Environment::new(&["sh"], "rootfs");
        assert!(&environment.hostname().is_none());
    }

    #[test]
    fn environment_set_hostname_returns_error_without_uts_namespace() {
        let mut environment = Environment::new(&["sh"], "rootfs");

        let set_hostname_result = environment.set_hostname("test");

        assert!(set_hostname_result.is_err());
        assert_eq!(environment.hostname(), &None);

    }

    #[test]
    fn environment_set_namespaces() {
        let mut environment = Environment::new(&["sh"], "rootfs");

        let mut namespaces = NamespaceList::empty();
        namespaces.insert(Namespace::new(NamespaceType::PID, None)).unwrap();

        environment.set_namespaces(namespaces);
        assert_eq!(environment.namespaces().as_vec().len(), 1);
    }

    #[test]
    fn environment_set_env_var() {
        let environment = Environment::new(&["sh"], "rootfs");

        environment.set_env_var("MY_VAR", "some_value").unwrap();

        assert_eq!(std::env::var("MY_VAR").unwrap(), "some_value");
    }
}
