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
use std::process::Command;
use std::process::Child;
use std::os::unix::process::CommandExt;

pub struct PosixProcess {
    working_dir: WorkingDir,
    env_vars: EnvVars,
    limits: ResourceLimits,
    command: Command,
}

impl Process for PosixProcess {
    fn before_exec<F>(&mut self, mut func: F) where F: FnMut() -> Result<(), Error> + Send + Sync + 'static {
        unsafe {
            self.command.pre_exec(move || {
                func()?;
                Ok(())
            });
        };
    }

    fn spawn(&mut self) -> Result<Child, Error> {
        let limits = self.limits.clone();
        let env_vars = self.env_vars.clone();
        let working_dir = self.working_dir.clone();
        let exec_fn = move || {
            limits.set()?;
            env_vars.set();
            working_dir.set()?;
            Ok(())
        };
        unsafe { self.command.pre_exec(exec_fn); }
        let child = self.command.spawn().context(format!("cannot exec container process"))?;
        Ok(child)
    }
}

impl FromSpec<PosixSpec> for PosixProcess {
    fn from_spec(spec: PosixSpec) -> Result<PosixProcess, Error> {
        let mut args = spec.process().args().to_vec();
        let mut command = Command::new(args.remove(0));
        command.args(args);

        let user = PosixUser::from_spec(spec.clone())?;
        command.uid(user.get_uid());
        command.gid(user.get_gid());

        let limits = ResourceLimits::from_spec(spec.clone())?;
        let env_vars = EnvVars::from_spec(spec.clone())?;
        let working_dir = WorkingDir::from_spec(spec.clone())?;

        Ok(PosixProcess { command, limits, env_vars, working_dir })
    }
}
