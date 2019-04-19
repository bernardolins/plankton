use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    args: Vec<String>,
    cwd: String,
}

impl Process {
    pub fn args(&self) -> &Vec<String> { &self.args }
    pub fn cwd(&self) -> &str { &self.cwd }
}
