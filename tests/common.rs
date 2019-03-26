extern crate tempfile;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::BufReader;
use self::tempfile::TempDir;

#[allow(dead_code)]
pub enum ConfigTemplate {
    Valid,
    NoHostname,
    SyntaxError,
    Invalid,
}

#[allow(dead_code)]
impl ConfigTemplate {
    fn file(&self) -> File {
        let file_name = match *self {
            ConfigTemplate::Valid => "valid_config.json",
            ConfigTemplate::NoHostname => "no_hostname_config.json",
            ConfigTemplate::SyntaxError => "syntax_error_config.json",
            ConfigTemplate::Invalid => "invalid_config.json",
        };

        let templates_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/templates"));
        let template_path = templates_path.join(file_name);
        File::open(template_path).expect("missing template file")
    }
}

#[allow(dead_code)]
pub struct TestBundle {
    pub dir: TempDir,
    pub path: PathBuf,
}

#[allow(dead_code)]
impl TestBundle {
    pub fn new(template: ConfigTemplate) -> TestBundle {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = PathBuf::from(dir.path());
        let config_path = dir_path.join("config.json");

        println!("{:?}", config_path);

        let mut config_file = File::create(config_path).unwrap();
        let mut template_content = BufReader::new(template.file());
        io::copy(&mut template_content, &mut config_file).unwrap();

        TestBundle {
            dir: dir,
            path: dir_path,
        }
    }

    pub fn empty() -> TestBundle {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = PathBuf::from(dir.path());

        TestBundle {
            dir: dir,
            path: dir_path,
        }
    }

    pub fn path(&self) -> &PathBuf { &self.path }
    pub fn str_path(&self) -> &str { &self.path.to_str().unwrap() }
}
