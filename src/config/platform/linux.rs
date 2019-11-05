use crate::spec::LinuxSpec;
use crate::spec::NamespaceSpec;
use serde::Serialize;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Linux {
    namespaces: Option<Vec<Namespace>>,
}

impl LinuxSpec for Linux {
    type Namespace = Namespace;

    fn get_namespaces(&self) -> Option<&Vec<Self::Namespace>> {
        self.namespaces.as_ref()
    }

    fn get_namespaces_clone(&self) -> Option<Vec<Self::Namespace>> {
        self.get_namespaces().cloned()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Namespace {
    r#type: String,
    path: Option<PathBuf>,
}

impl NamespaceSpec for Namespace {
    fn get_type(&self) -> &String {
        &self.r#type
    }

    fn get_type_clone(&self) -> String {
        self.r#type.clone()
    }

    fn get_path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    fn get_path_clone(&self) -> Option<PathBuf> {
        self.get_path().cloned()
    }
}
