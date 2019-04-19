use std::convert::TryFrom;

use super::Environment;

use crate::libcontainer::Config;
use crate::libcontainer::error::Error;

impl TryFrom<Config> for Environment {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let argv = config.process().args();
        let rootfs = config.root().path();

        let environment = Environment::new(&argv[..], rootfs);

        Ok(environment)
    }
}
