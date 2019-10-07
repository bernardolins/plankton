use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PosixUser {
    uid: i64,
    gid: i64,
    additional_gids: Option<Vec<i64>>,
}

impl PosixUser {
    fn default() -> PosixUser {
        PosixUser {
            uid: 0,
            gid: 0,
            additional_gids: None,
        }
    }
}

impl FromSpec<PosixSpec> for PosixUser {
    fn from_spec(spec: PosixSpec) -> Result<PosixUser, Error> {
        if let Some(user) = spec.process().user() {
            return Ok(PosixUser { uid: user.uid(), gid: user.gid(), additional_gids: user.additional_gids().clone() });
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

    #[test]
    fn default_returns_a_user_with_none_additional_gids() {
        let user = PosixUser::default();
        assert!(user.additional_gids.is_none());
    }
}
