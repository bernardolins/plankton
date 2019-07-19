pub mod resource;

use std::io;
use crate::libcontainer::Error;
use failure::ResultExt;

pub use self::resource::ResourceType;

#[derive(Debug)]
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

