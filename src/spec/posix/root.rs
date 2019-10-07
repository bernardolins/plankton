use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,

    #[serde(default = "Root::default_readonly")]
    pub readonly: bool,
}

impl Root {
    pub fn path(&self) -> &str { &self.path }
    pub fn readonly(&self) -> bool { self.readonly }

    fn default_readonly() -> bool { false }
}
