extern crate cr7;
extern crate tempfile;

mod common;

use cr7::container::Container;
use tempfile::TempDir;

fn setup_bundle(config_template: &str) -> TempDir {
    let tempdir = common::dir();
    let mut tempfile = common::file("config.json", &tempdir);
    let mut template = common::read_template(config_template);
    common::write_to(&mut tempfile, &mut template);
    tempdir
}

#[test]
fn creates_container_from_valid_config() {
    let bundle_dir = setup_bundle("valid_config.json");
    let bundle_path = bundle_dir.path().to_str().unwrap();
    let container = Container::new("some-container", bundle_path);
    assert!(container.is_ok(), "expect {:?} to be ok", container)
}

#[test]
fn sets_container_id() {
    let bundle_dir = setup_bundle("valid_config.json");
    let bundle_path = bundle_dir.path().to_str().unwrap();
    let container = Container::new("some-container", bundle_path);
    assert_eq!(container.unwrap().id(), "some-container")
}
