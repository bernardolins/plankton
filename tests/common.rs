extern crate tempfile;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::BufReader;
use tempfile::TempDir;

pub enum ConfigTemplate {
    Valid,
    NoHostname,
}

impl ConfigTemplate {
    fn file(&self) -> File {
        let file_name = match *self {
            ConfigTemplate::Valid => "valid_config.json",
            ConfigTemplate::NoHostname => "no_hostname_config.json"
        };

        let templates_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/templates"));
        let template_path = templates_path.join(file_name);
        File::open(template_path).unwrap()
    }
}


pub struct Bundle {
    pub dir: TempDir,
    pub path: PathBuf,
}

impl Bundle {
    pub fn new(template: ConfigTemplate) -> Bundle {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = PathBuf::from(dir.path());
        let config_path = dir_path.join("config.json");

        let mut config_file = File::create(config_path).unwrap();
        let mut template_content = BufReader::new(template.file());
        io::copy(&mut template_content, &mut config_file).unwrap();

        Bundle {
            dir: dir,
            path: dir_path,
        }
    }

    pub fn empty() -> Bundle {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = PathBuf::from(dir.path());
        let config_path = dir_path.join("config.json");

        Bundle {
            dir: dir,
            path: dir_path,
        }
    }

    pub fn str_path(&self) -> &str { &self.path.to_str().unwrap() }
}
