use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    pub namespaces: Vec<Namespace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    r#type: String,
    path: Option<String>,
}

impl Namespace {
    pub fn ns_type(&self) -> &str {
        &self.r#type
    }

    pub fn path(&self) -> Option<&str> {
        if let Some(path) = &self.path {
            return Some(&path)
        }
        None
    }
}
