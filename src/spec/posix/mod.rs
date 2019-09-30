pub mod root;
pub mod process;
pub mod mount;

#[cfg(target_os = "linux")]
pub mod linux;

use std::io::Read;
use std::io::BufReader;
use serde::Serialize;
use serde::Deserialize;
use failure::ResultExt;
use crate::Error;

pub use self::linux::Namespace;

use crate::spec::Spec;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PosixSpec {
    oci_version: String,
    hostname: Option<String>,
    root: root::Root,
    process: process::Process,
    mounts: Vec<mount::Mount>,

    #[cfg(target_os = "linux")]
    linux: linux::Linux,
}

impl PosixSpec {
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn root(&self) -> &root::Root { &self.root }
    pub fn process(&self) -> &process::Process { &self.process }
    pub fn mounts(&self) -> &Vec<mount::Mount> { &self.mounts }


    #[cfg(target_os = "linux")]
    pub fn namespaces(&self) -> &Vec<Namespace> {
        &self.linux.namespaces
    }
}

impl Spec for PosixSpec {
    type RootSpec = root::Root;
    type MountSpec = mount::Mount;
    type ProcessSpec = process::Process;

    fn from_reader<R: Read>(reader: R) -> Result<PosixSpec, Error> {
        let bufreader = BufReader::new(reader);
        let spec: PosixSpec = serde_json::from_reader(bufreader).context("error parsing config file".to_string())?;
        Ok(spec)
    }

    fn hostname(&self) -> &Option<String> {
        &self.hostname
    }

    fn oci_version(&self) -> &str {
        &self.oci_version
    }

    fn root(&self) -> &Self::RootSpec {
        &self.root
    }

    fn process(&self) -> &Self::ProcessSpec {
        &self.process
    }

    fn mounts(&self) -> &Vec<Self::MountSpec> {
        &self.mounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posix_spec_from_reader_with_synxtax_error() {
        let data = r#"{ociVersion: "1.0.1-dev"}"#;

        let config = PosixSpec::from_reader(data.as_bytes());
        assert!(config.is_err(), "expect {:?} to be err", config);
    }

    #[test]
    fn posix_spec_from_reader_return_err_when_json_has_no_oci_version() {
        let data = r#"{"hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = PosixSpec::from_reader(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn posix_spec_from_reader_return_err_when_json_has_no_root() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = PosixSpec::from_reader(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn posix_spec_from_reader_return_err_when_json_has_no_process() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = PosixSpec::from_reader(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn posix_spec_from_reader_return_err_when_json_has_no_linux() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}]}"#;

        let result = PosixSpec::from_reader(data.as_bytes());
        assert!(result.is_err(), "expect {:?} to be err", result);
    }

    #[test]
    fn posix_spec_from_reader_return_ok_when_json_has_no_hostname() {
        let data = r#"{"ociVersion":"1.0.0","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = PosixSpec::from_reader(data.as_bytes());
        assert!(result.is_ok(), "expect {:?} to be err", result);
    }

    #[test]
    fn posix_spec_from_reader_return_ok_when_json_has_all_fields() {
        let data = r#"{"ociVersion":"1.0.0","hostname":"hostname","process":{"args":["sh"],"env":["TERM=xterm"],"cwd":"/tmp","rlimits":[{"type":"RLIMIT_NOFILE","hard":1024,"soft":1024}]},"root":{"path":"rootfs","readonly":true},"mounts":[{"destination":"/proc","type":"proc","source":"/proc"}],"linux":{"namespaces":[{"type":"pid"}]}}"#;

        let result = PosixSpec::from_reader(data.as_bytes());
        assert!(result.is_ok(), "expect {:?} to be err", result);
    }
}
