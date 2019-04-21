use std::convert::TryFrom;

use super::Environment;

use crate::libcontainer::Config;
use crate::libcontainer::Namespace;
use crate::libcontainer::NamespaceType;
use crate::libcontainer::error::Error;

impl TryFrom<Config> for Environment {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let argv = config.process().args();
        let rootfs = config.root().path();
        let mut environment = Environment::new(&argv[..], rootfs);

        let working_dir = config.process().cwd();
        environment.set_working_dir(working_dir)?;

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

        Ok(environment)
    }
}
