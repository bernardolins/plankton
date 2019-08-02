#[cfg(target_os = "linux")]
pub mod rlimit;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    args: Vec<String>,
    env: Option<Vec<String>>,
    cwd: String,

    #[cfg(target_os = "linux")]
    rlimits: Option<Vec<rlimit::Rlimit>>,
}

impl Process {
    pub fn args(&self) -> &Vec<String> { &self.args }
    pub fn env(&self) -> &Option<Vec<String>> { &self.env }
    pub fn cwd(&self) -> &str { &self.cwd }

    #[cfg(target_os = "linux")]
    pub fn rlimits(&self) -> &Option<Vec<rlimit::Rlimit>> { &self.rlimits }
}
