use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    destination: String,
    source: Option<String>,

    #[cfg(target_os = "linux")] r#type: Option<String>,
}

impl Mount {
    pub fn destination(&self) -> &str { &self.destination }

    pub fn source(&self) -> Option<&str> {
        if let Some(source) = &self.source {
            Some(&source)
        } else {
            None
        }
    }

    pub fn filesystem_type(&self) -> Option<&str> {
        if let Some(r#type) = &self.r#type {
            Some(&r#type)
        } else {
            None
        }
    }
}
