use serde::Deserialize;
use std::ffi;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(default = "Process::default_terminal")]
    pub terminal: bool,

    #[serde(default = "Process::default_args")]
    pub args: Vec<ffi::CString>,

    #[serde(default = "Process::default_env")]
    pub env: Vec<ffi::CString>,
}

impl Process {
    pub fn default() -> Process {
        Process {
            terminal: Process::default_terminal(),
            args: Process::default_args(),
            env: Process::default_env(),
        }
    }

    fn default_terminal() -> bool { true }
    fn default_args() -> Vec<ffi::CString> { vec!(ffi::CString::new("sh").unwrap()) }
    fn default_env() -> Vec<ffi::CString> { Vec::new() }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(default = "Root::default_path")]
    pub path: ffi::CString,

    #[serde(default = "Root::default_readonly")]
    pub readonly: bool,
}

impl Root {
    pub fn default() -> Root {
        Root {
            path: Root::default_path(),
            readonly: Root::default_readonly(),
        }
    }

    fn default_path() -> ffi::CString { ffi::CString::new("rootpath").unwrap() }
    fn default_readonly() -> bool { true }
}

#[cfg(test)]
mod tests {
    use oci::spec::{Process, Root};

    #[test]
    fn process_default() {
        let process = Process::default();
        assert_eq!(process.terminal, Process::default_terminal());
        assert_eq!(process.args, Process::default_args());
        assert_eq!(process.env, Process::default_env());
    }

    #[test]
    fn root_default() {
        let root = Root::default();
        assert_eq!(root.path, Root::default_path());
        assert_eq!(root.readonly, Root::default_readonly());
    }
}
