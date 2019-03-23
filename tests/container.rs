extern crate cr7;
extern crate tempfile;

mod common;

use cr7::container::Container;
use tempfile::TempDir;

use common::{Bundle, ConfigTemplate};

#[test]
fn test_create_container() {
    let bundle = Bundle::new(ConfigTemplate::Valid);
    let container = Container::new("some-container", bundle.str_path());

    assert!(container.is_ok(), "expect {:?} to be ok", container);
    assert_eq!(container.unwrap().id(), "some-container");
}
