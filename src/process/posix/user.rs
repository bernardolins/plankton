use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use failure::ResultExt;
use nix::unistd;
use nix::unistd::Uid;
use nix::unistd::Gid;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PosixUser {
    uid: u32,
    gid: u32,
}

impl PosixUser {
    fn default() -> PosixUser {
        PosixUser {
            uid: 0,
            gid: 0,
        }
    }

    pub fn get_uid(&self) -> u32 {
        self.uid
    }

    pub fn get_gid(&self) -> u32 {
        self.gid
    }
}

impl FromSpec<PosixSpec> for PosixUser {
    fn from_spec(spec: PosixSpec) -> Result<PosixUser, Error> {
        if let Some(user) = spec.process().user() {
            return Ok(PosixUser { uid: user.uid() as u32, gid: user.gid() as u32 });
        }
        Ok(PosixUser::default())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_returns_a_user_with_uid_0() {
        let user = PosixUser::default();
        assert_eq!(user.uid, 0);
    }

    #[test]
    fn default_returns_a_user_with_gid_0() {
        let user = PosixUser::default();
        assert_eq!(user.gid, 0);
    }
}
