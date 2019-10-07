mod rlimit;
mod user;

use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use crate::process::EnvVars;
use crate::process::WorkingDir;
use self::rlimit::ResourceLimits;
use self::user::PosixUser;

pub struct PosixProcess {
    args: Vec<String>,
    working_dir: WorkingDir,
    env_vars: EnvVars,
    limits: ResourceLimits,
    user: PosixUser,
}

impl FromSpec<PosixSpec> for PosixProcess {
    fn from_spec(spec: PosixSpec) -> Result<PosixProcess, Error> {
        let process = PosixProcess {
            args: spec.process().args().to_vec(),
            env_vars: EnvVars::from_spec(spec.clone())?,
            limits: ResourceLimits::from_spec(spec.clone())?,
            user: PosixUser::from_spec(spec.clone())?,
            working_dir: WorkingDir::from_spec(spec.clone())?,
        };

        Ok(process)
    }
}
