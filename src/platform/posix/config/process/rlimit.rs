use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    r#type: String,
    soft: u64,
    hard: u64,
}

impl Rlimit {
    pub fn rl_type(&self) -> &str { &self.r#type }
    pub fn soft(&self) -> u64 { self.soft }
    pub fn hard(&self) -> u64 { self.hard }
}
