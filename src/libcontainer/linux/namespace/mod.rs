pub mod error;
pub mod r#type;
pub mod list;

use nix::sched;
use nix::sched::CloneFlags;

use crate::Error;

pub use self::error::ErrorKind;
pub use self::r#type::NamespaceType;
pub use self::list::NamespaceList;

#[derive(Debug)]
pub struct Namespace {
    r#type: NamespaceType,
    path: Option<String>,
}

impl Namespace {
    pub fn new(r#type: NamespaceType, path: Option<String>) -> Namespace {
        Namespace {
            r#type: r#type,
            path: path,
        }
    }

    pub fn enter(&self) -> Result<(), Error> {
        if let None = self.path {
            sched::unshare(self.unshare_flags())?;
        }
        Ok(())
    }

    fn unshare_flags(&self) -> CloneFlags {
        let mut flags = CloneFlags::empty();
        match self.r#type {
            NamespaceType::PID => flags.insert(CloneFlags::CLONE_NEWPID),
            NamespaceType::UTS => flags.insert(CloneFlags::CLONE_NEWUTS),
            NamespaceType::IPC => flags.insert(CloneFlags::CLONE_NEWIPC),
            NamespaceType::USER => flags.insert(CloneFlags::CLONE_NEWUSER),
            NamespaceType::MOUNT => flags.insert(CloneFlags::CLONE_NEWNS),
            NamespaceType::CGROUP => flags.insert(CloneFlags::CLONE_NEWCGROUP),
            NamespaceType::NETWORK => flags.insert(CloneFlags::CLONE_NEWNET),
        }

        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_new_allows_none_paths() {
        let namespace = Namespace::new(NamespaceType::UTS, None);
        assert!(namespace.path.is_none());
    }

    #[test]
    fn namespace_new_allows_some_paths() {
        let namespace = Namespace::new(NamespaceType::UTS, Some(String::from("/proc/1234/ns/uts")));
        assert!(namespace.path.is_some());
    }
}
