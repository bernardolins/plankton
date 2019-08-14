use crate::Error;
use crate::bundle;
use crate::libcontainer::Namespace;
use crate::libcontainer::NamespaceType;
use crate::libcontainer::MountPoint;
use crate::libcontainer::linux::rlimit::Rlimit;
use crate::libcontainer::linux::rlimit::ResourceType;
use std::path::PathBuf;
use super::Environment;

impl Environment {
    pub fn build(bundle_dir: &str) -> Result<Environment, Error> {
        let config = bundle::load_config(bundle_dir)?;

        let argv = config.process().args();
        let rootfs = PathBuf::from(bundle_dir).join(config.root().path());
        let mut environment = Environment::new(&argv[..], rootfs);

        let working_dir = config.process().cwd();
        environment.set_working_dir(working_dir)?;

        if let Some(env_vars) = config.process().env() {
            for env_var in env_vars {
                environment.add_env_var(env_var)?;
            }
        }

        for namespace_config in config.namespaces() {
            let str_type = namespace_config.ns_type();
            let path = match namespace_config.path() {
                Some(str_path) => Some(str_path.to_string()),
                None => None,
            };

            let ns_type = NamespaceType::from_str(str_type)?;
            let namespace = Namespace::new(ns_type, path);
            environment.set_namespace(namespace)?;
        }

        if let Some(hostname) = config.hostname() {
            environment.set_hostname(hostname)?;
        }

        for mount in config.mounts() {
            let source = mount.source();
            let destination = mount.destination();
            let filesystem_type = mount.filesystem_type();

            let mount_point = MountPoint::create(source, destination, filesystem_type);
            environment.add_mount_point(mount_point);
        }

        if let Some(rlimits) = config.process().rlimits() {
            for rlimit in rlimits {
                let resource = ResourceType::from_str(rlimit.rl_type())?;
                let rlim = Rlimit::new(resource, rlimit.soft(), rlimit.hard());
                environment.add_rlimit(rlim);
            }
        }

        Ok(environment)
    }
}
