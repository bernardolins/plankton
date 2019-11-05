use crate::spec::RootSpec;
use serde::Serialize;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Root {
    path: PathBuf,
    readonly: Option<bool>,
}

impl RootSpec for Root {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }

    fn get_path_clone(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_readonly(&self) -> Option<&bool> {
        self.readonly.as_ref()
    }

    fn get_readonly_clone(&self) -> Option<bool> {
        self.readonly.as_ref().cloned()
    }
}
