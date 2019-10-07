use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    args: Vec<String>,
    env: Option<Vec<String>>,
    cwd: String,
    rlimits: Option<Vec<Rlimit>>,
    user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    r#type: String,
    soft: u64,
    hard: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    uid: i64,
    gid: i64,
    additional_gids: Option<Vec<i64>>,
}

impl Process {
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn env(&self) -> &Option<Vec<String>> {
        &self.env
    }

    pub fn cwd(&self) -> &str {
        &self.cwd
    }

    pub fn rlimits(&self) -> &Option<Vec<Rlimit>> {
        &self.rlimits
    }

    pub fn user(&self) -> &Option<User> {
        &self.user
    }
}

impl Rlimit {
    pub fn rl_type(&self) -> &str {
        &self.r#type
    }

    pub fn soft(&self) -> u64 {
        self.soft
    }

    pub fn hard(&self) -> u64 {
        self.hard
    }
}

impl User {
    pub fn uid(&self) -> i64 {
        self.uid
    }

    pub fn gid(&self) -> i64 {
        self.gid
    }

    pub fn additional_gids(&self) -> &Option<Vec<i64>> {
        &self.additional_gids
    }
}
