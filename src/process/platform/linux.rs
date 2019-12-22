use crate::Error;
use crate::namespace::NamespaceSet;
use crate::process::ProcessCreate;
use crate::spec::ProcessSpec;
use crate::spec::LinuxSpec;
use crate::spec::ConsoleSizeSpec;
use failure::ResultExt;
use std::ffi::CString;
use std::ffi::OsString;
use std::path::PathBuf;

trait LinuxProcess {
    fn namespaces<L: LinuxSpec>(&mut self, spec: &L) -> Result<(), Error>;
}

#[derive(Debug, PartialEq)]
pub struct Process {
    program: Option<CString>,
    args: Option<Vec<CString>>,
    attach_terminal: bool,
    console_height: Option<u16>,
    console_width: Option<u16>,
    cwd: PathBuf,
    envs: Vec<(OsString, OsString)>,
    namespaces: NamespaceSet,
}

impl LinuxProcess for Process {
    fn namespaces<L: LinuxSpec>(&mut self, spec: &L) -> Result<(), Error> {
        let ns = spec.get_namespaces();
        self.namespaces = NamespaceSet::from_spec(ns)?;
        Ok(())
    }
}

impl ProcessCreate for Process {
    fn from_spec<P: ProcessSpec>(spec: &P) -> Result<Process, Error> {
        let mut process = Process {
            program: None,
            args: None,
            attach_terminal: false,
            console_height: None,
            console_width: None,
            cwd: PathBuf::from("/"),
            envs: Vec::new(),
            namespaces: NamespaceSet::empty(),
        };
        process.args(spec.get_args_clone())?;
        process.attach_terminal(spec.get_terminal());
        process.cwd(spec.get_cwd_clone())?;
        process.console_size(spec.get_console_size_clone());
        process.envs(spec.get_env_clone())?;
        Ok(process)
    }
}

impl Process {
    fn args(&mut self, args: Option<Vec<String>>) -> Result<(), Error> {
        if args.is_some() {
            let args = args.unwrap();
            let mut cstring_args = Vec::<CString>::new();
            for arg in args {
                let cstring_arg = CString::new(arg).context(format!("invalid argument on args list"))?;
                cstring_args.push(cstring_arg);
            }
            self.program = cstring_args.first().cloned();
            self.args = Some(cstring_args);
        }
        Ok(())
    }

    fn attach_terminal(&mut self, attach: Option<&bool>) {
        self.attach_terminal = attach.is_some() && *attach.unwrap();
    }

    fn console_size<S: ConsoleSizeSpec>(&mut self, size: Option<S>) {
        if self.attach_terminal && size.is_some() {
            let s = size.unwrap();
            self.console_width = Some(s.get_width());
            self.console_height = Some(s.get_height());
        }
    }

    fn cwd(&mut self, dir: PathBuf) -> Result<(), Error> {
        if dir.is_relative() {
            Err(Error::from("must be an absolute path")).context(format!("{:?}", dir))?;
        }
        self.cwd = dir;
        Ok(())
    }

    pub fn envs(&mut self, envs: Option<Vec<String>>) -> Result<(), Error> {
        if envs.is_some() {
            for env in envs.unwrap() {
                self.put_env(&env)?;
            }
        }
        Ok(())
    }

    fn put_env(&mut self, env: &str) -> Result<(), Error> {
        let mut splitted_env: Vec<&str> = env.split("=").collect();
        let error_message = "environment variable must have 'KEY=VALUE' format";
        if splitted_env.len() != 2 {
            Err(Error::from(error_message)).context(env.to_string())?
        }
        let k = OsString::from(splitted_env.remove(0));
        let v = OsString::from(splitted_env.remove(0));

        if k.is_empty() {
            Err(Error::from(error_message)).context(env.to_string())?
        }
        self.envs.push((k, v));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::ProcessSpec;
    use crate::spec::ConsoleSizeSpec;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[derive(Debug)]
    struct FakeProcess {
        fake_terminal: Option<bool>,
        fake_console_size: Option<FakeConsoleSize>,
        fake_cwd: PathBuf,
        fake_env: Option<Vec<String>>,
        fake_args: Option<Vec<String>>,
    }

    #[derive(Clone, Debug)]
    struct FakeConsoleSize {
        fake_width: u16,
        fake_height: u16,
    }

    impl ProcessSpec for FakeProcess {
        type ConsoleSize = FakeConsoleSize;

        fn get_terminal(&self) -> Option<&bool> { self.fake_terminal.as_ref() }
        fn get_terminal_clone(&self) -> Option<bool> { self.get_terminal().cloned() }
        fn get_console_size(&self) -> Option<&Self::ConsoleSize> { self.fake_console_size.as_ref() }
        fn get_console_size_clone(&self) -> Option<Self::ConsoleSize> { self.get_console_size().cloned() }
        fn get_cwd(&self) -> &PathBuf { &self.fake_cwd }
        fn get_cwd_clone(&self) -> PathBuf { self.fake_cwd.clone() }
        fn get_env(&self) -> Option<&Vec<String>> { self.fake_env.as_ref() }
        fn get_env_clone(&self) -> Option<Vec<String>> { self.get_env().cloned() }
        fn get_args(&self) -> Option<&Vec<String>> { self.fake_args.as_ref() }
        fn get_args_clone(&self) -> Option<Vec<String>> { self.get_args().cloned() }
    }

    impl ConsoleSizeSpec for FakeConsoleSize {
        fn get_height(&self) -> u16 { self.fake_width }
        fn get_width(&self) -> u16 { self.fake_height }
    }

    fn empty_spec() -> FakeProcess {
        FakeProcess {
            fake_terminal: None,
            fake_console_size: None,
            fake_cwd: PathBuf::from("/"),
            fake_env: None,
            fake_args: None,
        }
    }

    fn to_cstring(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    fn to_osstring_vec(items: Vec<(&str, &str)>) -> Vec<(OsString, OsString)> {
        let mut result: Vec<(OsString, OsString)> = Vec::new();
        for (k, v) in items {
            let os_k = OsString::from(k);
            let os_v = OsString::from(v);
            result.push((os_k, os_v));
        }
        result
    }

    #[test]
    fn from_spec_args() {
        let table = vec![
            (
                None,
                None,
                true
            ),
            (
                Some(vec!["/bin/sh".to_string()]),
                Some(vec![to_cstring("/bin/sh")]),
                true
            ),
            (
                Some(vec!["/bin/ping".to_string(), "localhost".to_string()]),
                Some(vec![to_cstring("/bin/ping"), to_cstring("localhost")]),
                true
            ),
            (
                Some(vec!["/bi\0n/ping".to_string(), "localhost".to_string()]),
                None,
                false
            ),
        ];
        for (spec_value, expected_args, is_ok) in table {
            let mut spec = empty_spec();
            spec.fake_args = spec_value;
            let result = Process::from_spec(&spec);
            assert_eq!(result.is_ok(), is_ok, "expected result.is_ok() to be {:?}, but got {:?}", is_ok, &result);
            if result.is_ok() {
                let proc = result.unwrap();
                assert_eq!(proc.args, expected_args, "expected process args to be {:?} but got {:?}", expected_args, proc.args);
            }
        }
    }

    #[test]
    fn from_spec_attach_terminal() {
        let table = vec![
            (Some(true), true),
            (Some(false), false),
            (None, false),
        ];
        for (spec_value, expected) in table {
            let mut spec = empty_spec();
            spec.fake_terminal = spec_value;
            let proc = Process::from_spec(&spec).unwrap();
            assert_eq!(proc.attach_terminal, expected, "expected process attach_terminal to be {:?} but got {:?}", proc.attach_terminal, expected)
        }
    }

    #[test]
    fn from_spec_working_dir() {
        let table = vec![
            (
                PathBuf::from(""),
                PathBuf::from("/"),
                false
            ),
            (
                PathBuf::from("/tmp"),
                PathBuf::from("/tmp"),
                true
            ),
            (
                PathBuf::from("./tmp"),
                PathBuf::from("./tmp"),
                false
            ),
            (
                PathBuf::from("C:\\Windows"),
                PathBuf::from("/"),
                false
            )
        ];
        for (spec_value, expected, is_ok) in table {
            let mut spec = empty_spec();
            spec.fake_cwd = spec_value;
            let result = Process::from_spec(&spec);
            assert_eq!(result.is_ok(), is_ok, "expected result.is_ok() to be {:?}, but got {:?}", is_ok, &result);
            if result.is_ok() {
                let proc = result.unwrap();
                assert_eq!(proc.cwd, expected, "expected process cwd to be {:?} but got {:?}", expected, proc.cwd);
            }
        }
    }

    #[test]
    fn from_spec_env_vars() {
        let table = vec![
            (
                None,
                vec![],
                true
            ),
            (
                Some(vec![]),
                vec![],
                true
            ),
            (
                Some(vec!["=TERM=xterm".to_string()]),
                vec![],
                false
            ),
            (
                Some(vec!["TERM=xterm".to_string(), "=PATH=/".to_string()]),
                vec![],
                false
            ),
            (
                Some(vec!["=xterm".to_string()]),
                vec![],
                false
            ),
            (
                Some(vec!["PATH=".to_string()]),
                to_osstring_vec(vec![("PATH", "")]),
                true
            ),
            (
                Some(vec!["=".to_string()]),
                vec![],
                false
            ),
            (
                Some(vec!["PATH=/bin:/usr/bin".to_string()]),
                to_osstring_vec(vec![("PATH", "/bin:/usr/bin")]),
                true
            ),
        ];
        for (spec_value, expected, is_ok) in table {
            let mut spec = empty_spec();
            spec.fake_env = spec_value;
            let result = Process::from_spec(&spec);
            assert_eq!(result.is_ok(), is_ok, "expected result.is_ok() to be {:?}, but got {:?}: spec value: {:?}", is_ok, &result, &spec.fake_env);
            if result.is_ok() {
                let proc = result.unwrap();
                assert_eq!(proc.envs, expected, "expected process envs to be {:?} but got {:?}", expected, proc.envs);
            }
        }
    }
}
