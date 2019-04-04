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

    pub fn working_dir(&mut self, working_dir: &str) {
        self.working_dir = Some(PathBuf::from(working_dir));
    }

    pub fn hostname(&mut self, hostname: &str) {
        self.hostname = Some(String::from(hostname));
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

        environment.working_dir("/");
        assert_eq!(environment.working_dir.unwrap(), PathBuf::from("/"));

    }

    #[test]
    fn environment_hostname() {
        let mut environment = Environment::new(&["sh"], "rootfs");
        assert!(&environment.hostname.is_none());

        environment.hostname("test");
        assert_eq!(&environment.hostname.unwrap(), "test");

    }
}
