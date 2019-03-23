extern crate tempfile;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::BufReader;
use tempfile::TempDir;

pub fn dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

pub fn file(name: &str, dir: &TempDir) -> File {
    let path = PathBuf::from(dir.path());
    let file_path = path.join(name);
    File::create(file_path).unwrap()
}

pub fn read_template(template_name: &str) -> BufReader<File> {
    let templates_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/templates"));
    let template_path = templates_path.join(template_name);
    let template = File::open(template_path).unwrap();
    BufReader::new(template)
}

pub fn write_to(file: &mut File, contents: &mut BufReader<File>) {
    io::copy(contents, file).unwrap();
}
