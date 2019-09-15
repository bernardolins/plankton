use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    uid: i64,
    gid: i64,
    additional_gids: Option<Vec<i64>>,
}

impl User {
    pub fn new(uid: i64, gid: i64, additional_gids: Option<Vec<i64>>) -> User {
        User { uid, gid, additional_gids }
    }

    pub fn root() -> User {
        User {
            uid: 0,
            gid: 0,
            additional_gids: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_root_returns_a_user_with_given_parameters() {
        let user = User::new(100, 100, Some(vec![200, 300, 400]));
        assert_eq!(user.uid, 100);
        assert_eq!(user.uid, 100);
        assert!(user.additional_gids.is_some());
        assert_eq!(user.additional_gids.unwrap(), vec![200, 300, 400]);
    }

    #[test]
    fn user_root_returns_a_user_with_uid_0() {
        let user = User::root();
        assert_eq!(user.uid, 0);
    }

    #[test]
    fn user_root_returns_a_user_with_gid_0() {
        let user = User::root();
        assert_eq!(user.gid, 0);
    }

    #[test]
    fn user_root_returns_a_user_with_none_additional_gids() {
        let user = User::root();
        assert!(user.additional_gids.is_none());
    }
}
