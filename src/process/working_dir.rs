use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use failure::ResultExt;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
const DEFAULT: &str = "/";

#[cfg(not(target_os = "linux"))]
const DEFAULT: &str = "";

pub struct WorkingDir {
    dir: PathBuf,
}

impl From<&str> for WorkingDir {
    fn from(string: &str) -> WorkingDir {
        WorkingDir {
            dir: PathBuf::from(string),
        }
    }
}

impl WorkingDir {
    pub fn default() -> WorkingDir {
        WorkingDir::from(DEFAULT)
    }

    fn validate(&self) -> Result<(), Error> {
        if self.dir.is_relative() {
            Err(
                Error::from("working dir is not a valid absolute path".to_string())
            ).context(format!("{:?}", self.dir))?
        }
        Ok(())
    }
}

impl FromSpec<PosixSpec> for WorkingDir {
    fn from_spec(spec: PosixSpec) -> Result<WorkingDir, Error> {
        let working_dir = WorkingDir::from(spec.process().cwd());
        working_dir.validate()?;
        Ok(working_dir)
    }
}
