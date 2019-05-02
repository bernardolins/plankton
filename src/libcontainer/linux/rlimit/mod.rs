pub mod error;

use crate::libcontainer::Error;

pub use self::error::ErrorReason;
#[derive(Debug)]
pub enum ResourceType {
   RLIMIT_AS,
   RLIMIT_CORE,
   RLIMIT_CPU,
   RLIMIT_DATA,
   RLIMIT_FSIZE,
   RLIMIT_LOCKS,
   RLIMIT_MEMLOCK,
   RLIMIT_MSGQUEUE,
   RLIMIT_NICE,
   RLIMIT_NOFILE,
   RLIMIT_NPROC,
   RLIMIT_RSS,
   RLIMIT_RTPRIO,
   RLIMIT_RTTIME,
   RLIMIT_SIGPENDING,
   RLIMIT_STACK,
}

impl ResourceType {
    pub fn from_str(type_str: &str) -> Result<ResourceType, Error> {
        match type_str {
            "RLIMIT_AS" => Ok(ResourceType::RLIMIT_AS),
            "RLIMIT_CORE" => Ok(ResourceType::RLIMIT_CORE),
            "RLIMIT_CPU" => Ok(ResourceType::RLIMIT_CPU),
            "RLIMIT_DATA" => Ok(ResourceType::RLIMIT_DATA),
            "RLIMIT_FSIZE" => Ok(ResourceType::RLIMIT_FSIZE),
            "RLIMIT_LOCKS" => Ok(ResourceType::RLIMIT_LOCKS),
            "RLIMIT_MEMLOCK" => Ok(ResourceType::RLIMIT_MEMLOCK),
            "RLIMIT_MSGQUEUE" => Ok(ResourceType::RLIMIT_MSGQUEUE),
            "RLIMIT_NICE" => Ok(ResourceType::RLIMIT_NICE),
            "RLIMIT_NOFILE" => Ok(ResourceType::RLIMIT_NOFILE),
            "RLIMIT_NPROC" => Ok(ResourceType::RLIMIT_NPROC),
            "RLIMIT_RSS" => Ok(ResourceType::RLIMIT_RSS),
            "RLIMIT_RTPRIO" => Ok(ResourceType::RLIMIT_RTPRIO),
            "RLIMIT_RTTIME" => Ok(ResourceType::RLIMIT_RTTIME),
            "RLIMIT_SIGPENDING" => Ok(ResourceType::RLIMIT_SIGPENDING),
            "RLIMIT_STACK" => Ok(ResourceType::RLIMIT_STACK),
            _ => Err(Error::from(ErrorKind::new("invalid resource type"))),
        }
    }

    fn to_libc(&self) -> libc::c_int {
        match *self {
            ResourceType::RLIMIT_AS => libc::RLIMIT_AS,
            ResourceType::RLIMIT_CORE => libc::RLIMIT_CORE,
            ResourceType::RLIMIT_CPU => libc::RLIMIT_CPU,
            ResourceType::RLIMIT_DATA => libc::RLIMIT_DATA,
            ResourceType::RLIMIT_FSIZE => libc::RLIMIT_FSIZE,
            ResourceType::RLIMIT_LOCKS => libc::RLIMIT_LOCKS,
            ResourceType::RLIMIT_MEMLOCK => libc::RLIMIT_MEMLOCK,
            ResourceType::RLIMIT_MSGQUEUE => libc::RLIMIT_MSGQUEUE,
            ResourceType::RLIMIT_NICE => libc::RLIMIT_NICE,
            ResourceType::RLIMIT_NOFILE => libc::RLIMIT_NOFILE,
            ResourceType::RLIMIT_NPROC => libc::RLIMIT_NPROC,
            ResourceType::RLIMIT_RSS => libc::RLIMIT_RSS,
            ResourceType::RLIMIT_RTPRIO => libc::RLIMIT_RTPRIO,
            ResourceType::RLIMIT_RTTIME => libc::RLIMIT_RTTIME,
            ResourceType::RLIMIT_SIGPENDING => libc::RLIMIT_SIGPENDING,
            ResourceType::RLIMIT_STACK => libc::RLIMIT_STACK,
        }
    }
}

#[derive(Debug)]
pub struct Rlimit {
    pub resource: ResourceType,
    pub soft: u64,
    pub hard: u64,
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
                let err = io::Error::last_os_error();
                let kind = ErrorKind::new(&format!("{}", err));
                return Err(Error::from(kind));
            }
        }

        Ok(())
    }
}


#[derive(Debug, PartialEq)]
pub struct ErrorKind {
    message: String,
}

impl ErrorKind {
    pub fn new(message: &str) -> ErrorKind {
        ErrorKind {
            message: String::from(message),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rlimit error: {}", &self.message)
    }
}
