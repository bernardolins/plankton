extern crate cr7;
extern crate tempfile;

mod common;

use cr7::container::Container;
use cr7::error::Error;

use tempfile::TempDir;

use common::{Bundle, ConfigTemplate};

#[test]
fn missing_bundle_path() {
    let container = Container::new("some-container", "/invalid/bundle/path");
    assert!(container.is_err(), "expect {:?} to be err", container);
    assert_eq!(container.err().unwrap(), Error::NotFound)
}

fn missing_config_file() {
    let bundle = Bundle::empty();
    let container = Container::new("some-container", bundle.str_path());
    assert!(container.is_err(), "expect {:?} to be err", container);
    assert_eq!(container.err().unwrap(), Error::NotFound)
}

#[test]
fn create_container() {
    let bundle = Bundle::new(ConfigTemplate::Valid);
    let container = Container::new("some-container", bundle.str_path());
    assert!(container.is_ok(), "expect {:?} to be ok", container);
    assert_eq!(container.unwrap().id(), "some-container");
}
