mod mount;
mod process;
mod root;

pub use self::mount::Mount;
pub use self::process::Process;
pub use self::root::Root;

use crate::Error;
use crate::spec::Spec;
use failure::ResultExt;
use serde::Serialize;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    mounts: Option<Vec<Mount>>,
    process: Option<Process>,
    root: Option<Root>,
}

impl Config {
    fn read_json<R: Read>(reader: R) -> Result<Config, Error> {
        let spec: Config =
            serde_json::from_reader(reader)
                .context("error reading config file".to_string())?;
        Ok(spec)
    }
}

impl Spec for Config {
    type Mount = Mount;
    type Root = Root;
    type Process = Process;

    fn get_root(&self) -> Option<&Self::Root> {
        self.root.as_ref()
    }

    fn get_root_clone(&self) -> Option<Self::Root> {
        self.get_root().cloned()
    }

    fn get_mounts(&self) -> Option<&Vec<Self::Mount>> {
        self.mounts.as_ref()
    }

    fn get_mounts_clone(&self) -> Option<Vec<Self::Mount>> {
        self.get_mounts().cloned()
    }

    fn get_process(&self) -> Option<&Self::Process> {
        self.process.as_ref()
    }

    fn get_process_clone(&self) -> Option<Self::Process> {
        self.get_process().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_root_some() {
        let input = r#"{"root": {"path": "/"}}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_root();
        assert!(result.is_some(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_root_clone_some() {
        let input = r#"{"root": {"path": "/"}}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_root_clone();
        assert!(result.is_some(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_mounts_some() {
        let input = r#"{"mounts": []}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_mounts();
        assert!(result.is_some(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_mounts_clone_some() {
        let input = r#"{"mounts": []}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_mounts_clone();
        assert!(result.is_some(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_process_some() {
        let input = r#"{"process": {"cwd": "/"}}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_process();
        assert!(result.is_some(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_process_clone_some() {
        let input = r#"{"process": {"cwd": "/"}}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_process_clone();
        assert!(result.is_some(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_root_none() {
        let input = r#"{}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_root();
        assert!(result.is_none(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_root_clone_none() {
        let input = r#"{}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_root();
        assert!(result.is_none(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_mounts_none() {
        let input = r#"{}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_process();
        assert!(result.is_none(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_mounts_clone_none() {
        let input = r#"{}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_process();
        assert!(result.is_none(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_process_none() {
        let input = r#"{}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_process();
        assert!(result.is_none(), "expect {:?} to be some", &result);
    }

    #[test]
    fn get_process_clone_none() {
        let input = r#"{}"#;
        let config = Config::read_json(input.as_bytes()).unwrap();
        let result = config.get_process();
        assert!(result.is_none(), "expect {:?} to be some", &result);
    }

    mod read_json {
        use crate::config::Config;

        #[test]
        fn returns_error_on_invalid_root() {
            let input = r#"{"root": {}}"#;
            let result = Config::read_json(input.as_bytes());
            assert!(result.is_err(), "expect {:?} to be err", &result)
        }

        #[test]
        fn returns_error_on_invalid_process() {
            let input = r#"{"process": {}}"#;
            let result = Config::read_json(input.as_bytes());
            assert!(result.is_err(), "expect {:?} to be err", &result)
        }

        #[test]
        fn returns_error_on_invalid_mounts() {
            let input = r#"{"mounts": [{}]}"#;
            let result = Config::read_json(input.as_bytes());
            assert!(result.is_err(), "expect {:?} to be err", &result)
        }

        #[test]
        fn returns_error_on_json_syntax() {
            let input = r#"{"process": [ }"#;
            let result = Config::read_json(input.as_bytes());
            assert!(result.is_err(), "expect {:?} to be err", &result)
        }
    }
}
