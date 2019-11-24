use crate::Error;
use crate::spec::ProcessSpec;
use failure::ResultExt;
use std::ffi::CString;
use std::path::PathBuf;
use std::process::Command;
use std::os::unix::process::CommandExt;

#[derive(Debug)]
pub struct Process {
    args: Option<Vec<CString>>,
    attach_terminal: bool,
    working_dir: PathBuf,
}

impl Process {
    pub fn from_spec<P: ProcessSpec>(proc_spec: &P) -> Result<Process, Error> {
        let attach_terminal = !(proc_spec.get_terminal().is_none()) && *proc_spec.get_terminal().unwrap();
        let spec_args = proc_spec.get_args_clone();
        let args = into_cstring_vec(spec_args)?;
        let working_dir = proc_spec.get_cwd_clone();
        check_working_dir(&working_dir)?;
        Ok(Process{
            attach_terminal,
            args,
            working_dir,
        })
    }
}

fn into_cstring_vec(string_vec: Option<Vec<String>>) -> Result<Option<Vec<CString>>, Error> {
    if string_vec.is_none() {
        return Ok(None)
    }
    let mut cstring_vec: Vec<CString> = Vec::new();
    let vec = string_vec.unwrap();
    for elem in vec {
        let cstring = CString::new(elem).context("failed to parse argument")?;
        cstring_vec.push(cstring);
    }
    Ok(Some(cstring_vec))
}

fn check_working_dir(working_dir: &PathBuf) -> Result<(), Error> {
    if working_dir.is_relative() {
        Err(Error::from("must be an absolute path")).context(format!("{:?}", working_dir))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::spec::ProcessSpec;
    use crate::spec::ConsoleSizeSpec;
    use std::ffi::CString;
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
        fake_width: u32,
        fake_height: u32,
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
        fn get_height(&self) -> u32 { self.fake_width }
        fn get_width(&self) -> u32 { self.fake_height }
    }

    fn empty_spec() -> FakeProcess {
        FakeProcess {
            fake_terminal: None,
            fake_console_size: None,
            fake_cwd: PathBuf::from(""),
            fake_env: None,
            fake_args: None,
        }
    }

    fn to_cstring(s: &str) -> CString {
        CString::new(s).unwrap()
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
            assert_eq!(proc.attach_terminal, expected, "unexpected process {:?} for spec {:?}", proc, spec)
        }
    }

    #[test]
    fn from_spec_args() {
        let table = vec![
            (None, None),
            (Some(vec!["/bin/sh".to_string()]), Some(vec![to_cstring("/bin/sh")])),
            (Some(vec!["/bin/ping".to_string(), "localhost".to_string()]), Some(vec![to_cstring("/bin/ping"), to_cstring("localhost")])),
        ];
        for (spec_value, expected_args) in table {
            let mut spec = empty_spec();
            spec.fake_args = spec_value;
            let proc = Process::from_spec(&spec).unwrap();
            assert_eq!(proc.args, expected_args, "expected process args to be {:?} but got {:?}", expected_args, proc.args);
        }
    }

    #[test]
    fn from_spec_working_dir() {
        let table = vec![
            (PathBuf::from(""), PathBuf::from(""), false),
            (PathBuf::from("/tmp"), PathBuf::from("/tmp"), true),
            (PathBuf::from("./tmp"), PathBuf::from(""), false),
        ];
        for (spec_value, expected, is_ok) in table {
            let mut spec = empty_spec();
            spec.fake_cwd = spec_value;
            let result = Process::from_spec(&spec);
            assert_eq!(result.is_ok(), is_ok, "expected result.is_ok to be {}, but result was {:?}", is_ok, &result);
            if result.is_ok() {
                let proc = result.unwrap();
                assert_eq!(proc.working_dir, expected, "expected process args to be {:?} but got {:?}", expected, proc.working_dir);
            }
        }
    }
}
