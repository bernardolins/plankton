pub mod resource;

use std::io;
use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use serde::Deserialize;
use serde::Serialize;
use failure::ResultExt;

pub use self::resource::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlimit {
    resource: ResourceType,
    soft: u64,
    hard: u64,
}

impl Rlimit {
    pub fn new(resource: ResourceType, soft: u64, hard: u64) -> Rlimit {
        Rlimit {
            soft: soft,
            hard: hard,
            resource: resource,
        }
    }

    pub fn set(&self) -> Result<(), Error> {
        let mut rlimit = libc::rlimit {
            rlim_cur: self.soft as libc::rlim_t,
            rlim_max: self.hard as libc::rlim_t,
        };

        let resource = self.resource.to_libc();

        unsafe {
            if libc::setrlimit(resource, &mut rlimit) != 0 {
                Err(io::Error::last_os_error()).context(format!("error setting rlimit {:?}", self.resource))?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ResourceLimits {
    limits: Vec<Rlimit>,
}

impl ResourceLimits {
    pub fn empty() -> ResourceLimits {
        ResourceLimits {
            limits: Vec::new(),
        }
    }

    pub fn set(&self) -> Result<(), Error> {
        for limit in &self.limits {
            limit.set()?;
        }
        Ok(())
    }

    fn add_rlimit(&mut self, rlimit: Rlimit) {
        self.limits.push(rlimit)
    }
}

impl FromSpec<PosixSpec> for ResourceLimits {
    fn from_spec(spec: PosixSpec) -> Result<ResourceLimits, Error> {
        let mut limits = ResourceLimits::empty();
        if let Some(rlimits) = spec.process().rlimits() {
            for rlimit in rlimits {
                let resource = ResourceType::from_str(rlimit.rl_type())?;
                let rlim = Rlimit::new(resource, rlimit.soft(), rlimit.hard());
                limits.add_rlimit(rlim);
            }
        }
        Ok(limits)
    }
}
