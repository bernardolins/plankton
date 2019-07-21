pub mod error;

use std::path::PathBuf;
use crate::Error;
use crate::libcontainer::linux::rlimit::Rlimit;
use crate::libcontainer::linux::mount::MountPoint;
use crate::libcontainer::linux::namespace::Namespace;
use crate::libcontainer::linux::namespace::NamespaceType;
use crate::libcontainer::linux::namespace::NamespaceList;
use serde::Deserialize;
use serde::Serialize;
use failure::ResultExt;

pub use self::error::ErrorKind;

const DEFAULT_WORKING_DIR: &str = "/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    argv: Vec<String>,
    rootfs: PathBuf,
    working_dir: PathBuf,
    hostname: Option<String>,
    namespaces: NamespaceList,
    mount_list: Vec<MountPoint>,
    env_vars: Vec<(String, String)>,
    rlimits: Vec<Rlimit>,
}

impl Environment {
    pub fn new(argv: &[String], rootfs: &str) -> Environment {
        Environment {
            argv: argv.to_vec(),
            rootfs: PathBuf::from(rootfs),
            working_dir: PathBuf::from(DEFAULT_WORKING_DIR),
            hostname: None,
            namespaces: NamespaceList::empty(),
            mount_list: Vec::new(),
            env_vars: Vec::new(),
            rlimits: Vec::new(),
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

    pub fn mount_list(&self) -> &Vec<MountPoint> {
        &self.mount_list
    }

    pub fn env_vars(&self) -> &Vec<(String, String)> {
        &self.env_vars
    }

    pub fn rlimits(&self) -> &Vec<Rlimit> {
        &self.rlimits
    }

    pub fn set_working_dir(&mut self, working_dir: &str) -> Result<(), Error> {
        let cwd = PathBuf::from(working_dir);

        if cwd.is_relative() {
            Err(Error::from(ErrorKind::InvalidWorkingDir)).context(working_dir.to_string())?
        }

        self.working_dir = cwd;
        Ok(())
    }

    pub fn set_hostname(&mut self, hostname: &str) -> Result<(), Error> {
        if !self.namespaces.contains_type(&NamespaceType::UTS) {
            Err(ErrorKind::PrivateHostname)?
        }

        self.hostname = Some(String::from(hostname));
        Ok(())
    }

    pub fn set_namespace(&mut self, namespace: Namespace) -> Result<(), Error> {
        self.namespaces.insert(namespace)
    }

    pub fn add_mount_point(&mut self, mount_point: MountPoint) {
        self.mount_list.push(mount_point);
    }

    pub fn add_env_var(&mut self, env_var: &str) -> Result<(), Error> {
        let mut splitted_env: Vec<&str> = env_var.split("=").collect();

        if splitted_env.len() != 2 {
            Err(Error::from(ErrorKind::WrongEnvVarFormat)).context(env_var.to_string())?
        }

        let k = String::from(splitted_env.remove(0));
        let v = String::from(splitted_env.remove(0));

        if k.is_empty() {
            Err(Error::from(ErrorKind::WrongEnvVarFormat)).context(env_var.to_string())?
        }

        self.env_vars.push((k, v));

        Ok(())
    }

    pub fn add_rlimit(&mut self, rlimit: Rlimit) {
        self.rlimits.push(rlimit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::libcontainer::linux::namespace::Namespace;
    use crate::libcontainer::linux::namespace::NamespaceType;

    use crate::libcontainer::linux::rlimit::Rlimit;
    use crate::libcontainer::linux::rlimit::ResourceType;

    fn setup_environment() -> Environment {
        Environment::new(&["sh".to_string()], "rootfs")
    }

    #[test]
    fn environment_new() {
        let environment = setup_environment();

        assert_eq!(environment.argv, vec!["sh"]);
        assert_eq!(environment.rootfs, PathBuf::from("rootfs"));
    }

    #[test]
    fn environment_argv() {
        let environment = setup_environment();

        assert_eq!(environment.argv(), &["sh"]);
    }

    #[test]
    fn environment_working_dir_defaults_to_root() {
        let environment = setup_environment();

        assert_eq!(environment.working_dir(), &PathBuf::from("/"));
    }

    #[test]
    fn environment_namespaces_defaults_to_empty_list() {
        let environment = setup_environment();

        assert_eq!(environment.namespaces().as_vec().len(), 0);
    }

    #[test]
    fn environment_mount_list_defaults_to_empty_list() {
        let environment = setup_environment();
        let mount_list = environment.mount_list();

        assert!(mount_list.is_empty(), "expect {:?} to be empty", mount_list);
    }

    #[test]
    fn environment_rlimits_defaults_to_empty_list() {
        let environment = setup_environment();
        let rlimits = environment.rlimits();

        assert!(rlimits.is_empty(), "expect {:?} to be empty", rlimits);
    }

    #[test]
    fn environment_set_working_dir() {
        let mut environment = setup_environment();
        let result = environment.set_working_dir("/tmp");

        assert!(result.is_ok(), "expect {:?} to be ok", &result);
        assert_eq!(environment.working_dir(), &PathBuf::from("/tmp"));
    }

    #[test]
    fn environment_set_working_dir_relative_path() {
        let mut environment = setup_environment();
        let result = environment.set_working_dir("./");

        assert!(result.is_err());
    }

    #[test]
    fn environment_hostname_defaults_to_none() {
        let environment = setup_environment();
        assert!(&environment.hostname().is_none());
    }

    #[test]
    fn environment_set_hostname_returns_error_without_uts_namespace() {
        let mut environment = setup_environment();

        let set_hostname_result = environment.set_hostname("test");

        assert!(set_hostname_result.is_err());
        assert_eq!(environment.hostname(), &None);

    }

    #[test]
    fn environment_set_namespace() {
        let mut environment = setup_environment();
        let namespace = Namespace::new(NamespaceType::UTS, None);

        let result = environment.set_namespace(namespace);

        assert!(result.is_ok(), "expect {:?} to be ok", result);
    }

    #[test]
    fn environment_cant_set_same_namespace_twice() {
        let mut environment = setup_environment();

        let namespace1 = Namespace::new(NamespaceType::UTS, None);
        let namespace2 = Namespace::new(NamespaceType::UTS, None);

        let result1 = environment.set_namespace(namespace1);
        let result2 = environment.set_namespace(namespace2);

        assert!(result1.is_ok(), "expect {:?} to be ok", result1);
        assert!(result2.is_err(), "expect {:?} to be err", result2);
    }

    #[test]
    fn environment_add_mount_point() {
        let mut environment = setup_environment();
        let mount_point = MountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"));

        environment.add_mount_point(mount_point);

        assert_eq!(environment.mount_list().len(), 1);
    }

    #[test]
    fn environment_add_env_var() {
        let mut environment = setup_environment();

        environment.add_env_var("MY_VAR=some_value").unwrap();

        assert_eq!(environment.env_vars().len(), 1);

        for (key, val) in environment.env_vars() {
            assert_eq!(key, "MY_VAR");
            assert_eq!(val, "some_value");
        }
    }

    #[test]
    fn environment_add_env_var_returns_error_with_multiple_equals_to_signs() {
        let mut environment = setup_environment();

        let result = environment.add_env_var("MY_VAR=some_value=other_value");

        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn environment_add_env_var_returns_error_with_empty_key() {
        let mut environment = setup_environment();

        let result = environment.add_env_var("=some_value");

        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn environment_add_rlimit() {
        let mut environment = setup_environment();
        let rlimit = Rlimit::new(ResourceType::RLIMIT_AS, 1024, 1024);

        environment.add_rlimit(rlimit);

        assert_eq!(environment.rlimits().len(), 1);
    }
}
