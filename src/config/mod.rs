pub mod root;
pub mod process;
pub mod mount;

mod conv;

#[cfg(target_os = "linux")]
pub mod linux;

use crate::Error;
use std::io::BufRead;
use serde::{Serialize, Deserialize};

pub use self::linux::Namespace;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    oci_version: String,
    hostname: Option<String>,
    root: root::Root,
    process: process::Process,
    mounts: Vec<mount::Mount>,

    #[cfg(target_os = "linux")]
    linux: linux::Linux,
}

impl Config {
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn root(&self) -> &root::Root { &self.root }
    pub fn process(&self) -> &process::Process { &self.process }
    pub fn mounts(&self) -> &Vec<mount::Mount> { &self.mounts }


    #[cfg(target_os = "linux")]
    pub fn namespaces(&self) -> &Vec<Namespace> {
        &self.linux.namespaces
    }

    pub fn load<R: BufRead>(reader: R) -> Result<Config, Error> {
        let spec: Config = serde_json::from_reader(reader)?;
        Ok(spec)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::error::Error) -> Error {
        Error::new(&format!("config - {}", serde_error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_load_with_synxtax_error() {
        let data = r#"{ociVersion: "1.0.1-dev"}"#;

        let config = Config::load(data.as_bytes());
        assert!(config.is_err(), "expect {:?} to be err", config);
    }

    #[test]
    fn config_load_return_err_when_json_has_no_oci_version() {
        let data = r#"{"hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = Config::load(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn config_load_return_err_when_json_has_no_root() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = Config::load(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn config_load_return_err_when_json_has_no_process() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = Config::load(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn config_load_return_err_when_json_has_no_linux() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}]}"#;

        let result = Config::load(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn config_load_return_ok_when_json_has_no_hostname() {
        let data = r#"{"ociVersion":"1.0.0","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = Config::load(data.as_bytes());
        assert!(result.is_ok(), "expect {:?} to be err", result);
    }

    #[test]
    fn config_load_return_ok_when_json_has_all_fields() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = Config::load(data.as_bytes());
        assert!(result.is_ok(), "expect {:?} to be err", result);
    }
}
