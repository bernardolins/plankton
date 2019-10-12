mod rlimit;
mod user;

use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use crate::process::Process;
use crate::process::EnvVars;
use crate::process::WorkingDir;
use failure::ResultExt;
use self::rlimit::ResourceLimits;
use self::user::PosixUser;
use std::io::ErrorKind;
use std::io::Error as IOError;
use std::process::Command;
use std::os::unix::process::CommandExt;

#[derive(Clone)]
pub struct PosixProcess {
    bin: String,
    args: Vec<String>,
    working_dir: WorkingDir,
    env_vars: EnvVars,
    limits: ResourceLimits,
    user: PosixUser,
}

impl Process for PosixProcess {
    type ProcessID = u32;

    fn spawn(&self) -> Result<Self::ProcessID, Error> {
        let proc = self.clone();
        let mut exec_fn = move || {
            let to_io_err = |err| {
                return IOError::new(ErrorKind::Other, format!("{:?}", err));
            };
            proc.env_vars.set();
            proc.limits.set().map_err(to_io_err)?;
            proc.working_dir.set().map_err(to_io_err)?;
            Ok(())
        };
        let child = unsafe {
            let mut cmd = Command::new(&self.bin);
            cmd.args(&self.args);
            cmd.pre_exec(exec_fn);
            cmd.uid(self.user.get_uid());
            cmd.gid(self.user.get_gid());
            let child = cmd.spawn().context(format!("{:?}", self.args))?;
            child
        };
        Ok(child.id())
    }
}

impl FromSpec<PosixSpec> for PosixProcess {
    fn from_spec(spec: PosixSpec) -> Result<PosixProcess, Error> {
        let mut args = spec.process().args().to_vec();
        let bin = args.remove(0);
        let process = PosixProcess {
            bin: String::from(bin),
            args: args,
            env_vars: EnvVars::from_spec(spec.clone())?,
            limits: ResourceLimits::from_spec(spec.clone())?,
            user: PosixUser::from_spec(spec.clone())?,
            working_dir: WorkingDir::from_spec(spec.clone())?,
        };

        Ok(process)
    }
}
