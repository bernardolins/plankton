use std::path::PathBuf;

pub struct Environment {
    argv: Vec<String>,
    rootfs: PathBuf,
    working_dir: Option<PathBuf>,
    hostname: Option<String>,
}

impl Environment {
    pub fn new(argv: &[&str], rootfs: &str) -> Environment {
        Environment {
            argv: argv.iter().map(|arg| String::from(*arg)).collect(),
            rootfs: PathBuf::from(rootfs),
            working_dir: None,
            hostname: None,
        }
    }

    pub fn working_dir(&mut self, working_dir: &str) -> Result<(), Error> {
        let cwd = PathBuf::from(working_dir);

        if cwd.is_absolute() {
            self.working_dir = Some(cwd);
            Ok(())
        } else {
            Err(Error::WorkingDir)
        }
    }

    pub fn hostname(&mut self, hostname: &str) {
        self.hostname = Some(String::from(hostname));
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    WorkingDir,
    Hostname,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match *self {
            Error::WorkingDir => "container working dir must be a valid absolute path",
            Error::Hostname => "container needs a private namespace in order to set hostname",
        };
        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn environment_new() {
        let environment = Environment::new(&["sh"], "rootfs");
        assert_eq!(environment.argv, vec!["sh"]);
        assert_eq!(environment.rootfs, PathBuf::from("rootfs"));
    }

    #[test]
    fn environment_working_dir() {
        let mut environment = Environment::new(&["sh"], "rootfs");
        assert!(&environment.working_dir.is_none());

        environment.working_dir("/").unwrap();
        assert_eq!(environment.working_dir.unwrap(), PathBuf::from("/"));

    }

    #[test]
    fn environment_working_dir_invalid() {
        let mut environment = Environment::new(&["sh"], "rootfs");

        let cwd = environment.working_dir("./");
        assert!(cwd.is_err());
        assert_eq!(cwd.err().unwrap(), Error::WorkingDir);
    }

    #[test]
    fn environment_hostname() {
        let mut environment = Environment::new(&["sh"], "rootfs");
        assert!(&environment.hostname.is_none());

        environment.hostname("test");
        assert_eq!(&environment.hostname.unwrap(), "test");

    }
}
