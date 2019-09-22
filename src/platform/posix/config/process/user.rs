use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    uid: i64,
    gid: i64,
    additional_gids: Option<Vec<i64>>,
}

impl User {
    pub fn uid(&self) -> i64 { self.uid }
    pub fn gid(&self) -> i64 { self.gid }
    pub fn additional_gids(&self) -> &Option<Vec<i64>> { &self.additional_gids }
}
