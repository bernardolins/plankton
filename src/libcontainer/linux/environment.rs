use std::path::PathBuf;

use crate::libcontainer::linux::namespace::Namespace;
use crate::libcontainer::linux::namespace::NamespaceType;
use crate::libcontainer::linux::namespace::NamespaceList;

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
    pub fn new(argv: &[&str], rootfs: &str) -> Environment {
        Environment {
            argv: argv.iter().map(|arg| String::from(*arg)).collect(),
            rootfs: PathBuf::from(rootfs),
            working_dir: PathBuf::from(DEFAULT_WORKING_DIR),
            hostname: None,
            namespaces: NamespaceList::empty(),
        }
    }

    pub fn argv(&self) -> &Vec<String> {
        &self.argv
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
        if self.namespaces.contains_type(NamespaceType::UTS) {
            self.hostname = Some(String::from(hostname));
            Ok(())
        } else {
            Err(Error::Hostname)
        }
    }

    pub fn set_namespaces(&mut self, ns_list: NamespaceList) {
        self.namespaces = ns_list;
    }

}

#[derive(Debug, PartialEq)]
pub enum Error {
    WorkingDir,
    Hostname,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            Error::WorkingDir => "container working dir must be a valid absolute path",
            Error::Hostname => "container needs a private UTS namespace in order to set hostname",
        };
        write!(f, "{}", message)
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
    fn environment_set_hostname() {
        let mut environment = Environment::new(&["sh"], "rootfs");

        environment.set_hostname("test");
        assert_eq!(environment.hostname(), &Some("test".to_string()));

    }

    #[test]
    fn environment_set_namespaces() {
        let mut environment = Environment::new(&["sh"], "rootfs");

        let mut namespaces = NamespaceList::empty();
        namespaces.insert(Namespace::new(NamespaceType::PID, None));

        environment.set_namespaces(namespaces);
        assert_eq!(environment.namespaces().as_vec().len(), 1);
    }
}
