use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    args: Vec<String>,
}

impl Process {
    pub fn args(&self) -> &Vec<String> { &self.args }
}
