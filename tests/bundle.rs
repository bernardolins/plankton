extern crate cr7;
extern crate rand;

mod common;

use cr7::bundle::Bundle;
use cr7::error::Error;

use common::{TestBundle, ConfigTemplate};

#[test]
fn bundle_path_missing() {
    let bundle = Bundle::new("/invalid/bundle/path");
    assert!(bundle.is_err(), "expect {:?} to be err", bundle);
    assert_eq!(bundle.err().unwrap(), Error::NotFound)
}

#[test]
fn config_file_missing() {
    let test_bundle = TestBundle::empty();
    let bundle = Bundle::new(test_bundle.str_path());
    assert!(bundle.is_err(), "expect {:?} to be err", bundle);
    assert_eq!(bundle.err().unwrap(), Error::NotFound)
}

#[test]
fn valid_bundle() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let bundle = Bundle::new(&test_bundle.str_path());
    assert!(bundle.is_ok(), "expect {:?} to be err", bundle);
}
