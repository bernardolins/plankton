use crate::spec::MountSpec;
use serde::Serialize;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Mount {
    destination: PathBuf,
    source: Option<String>,
    options: Option<Vec<String>>,

    #[cfg(target_os = "linux")]
    r#type: Option<String>,
}

impl MountSpec for Mount {
    fn get_destination(&self) -> &PathBuf {
        &self.destination
    }

    fn get_destination_clone(&self) -> PathBuf {
        self.destination.clone()
    }

    fn get_source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    fn get_source_clone(&self) -> Option<String> {
        self.source.as_ref().cloned()
    }

    fn get_options(&self) -> Option<&Vec<String>> {
        self.options.as_ref()
    }

    fn get_options_clone(&self) -> Option<Vec<String>> {
        self.options.as_ref().cloned()
    }

    #[cfg(target_os = "linux")]
    fn get_type(&self) -> Option<&String> {
        self.r#type.as_ref()
    }

    #[cfg(target_os = "linux")]
    fn get_type_clone(&self) -> Option<String> {
        self.r#type.as_ref().cloned()
    }
}
