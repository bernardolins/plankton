extern crate cr7;
extern crate rand;

mod common;

use cr7::container::Container;
use cr7::error::Error;

use common::{TestBundle, ConfigTemplate};

#[test]
fn bundle_path_missing() {
    let container = Container::create("some-container", "/invalid/bundle/path");
    assert!(container.is_err(), "expect {:?} to be err", container);
    assert_eq!(container.err().unwrap(), Error::NotFound)
}

#[test]
fn config_file_missing() {
    let bundle = TestBundle::empty();
    let container = Container::create("some-container", bundle.str_path());
    assert!(container.is_err(), "expect {:?} to be err", container);
    assert_eq!(container.err().unwrap(), Error::NotFound)
}

#[test]
fn config_file_synxtax_error() {
    let bundle = TestBundle::new(ConfigTemplate::SyntaxError);
    let container = Container::create("some-container", bundle.str_path());
    assert!(container.is_err(), "expect {:?} to be err", container);
    assert_eq!(container.err().unwrap(), Error::ConfigSyntax);
}

#[test]
fn config_file_invalid() {
    let bundle = TestBundle::new(ConfigTemplate::Invalid);
    let container = Container::create("some-container", bundle.str_path());
    assert!(container.is_err(), "expect {:?} to be err", container);
    assert_eq!(container.err().unwrap(), Error::ParseConfig);
}

#[test]
fn create_container() {
    let bundle = TestBundle::new(ConfigTemplate::Valid);
    let container_id = format!("container-{}", rand::random::<u32>());
    let container = Container::create(&container_id, bundle.str_path());
    assert!(container.is_ok(), "expect {:?} to be ok", container);
    assert_eq!(container.unwrap().id(), container_id);
}

#[test]
fn container_already_exist() {
    let bundle = TestBundle::new(ConfigTemplate::Valid);
    let container_id = format!("container-{}", rand::random::<u32>());
    Container::create(&container_id, bundle.str_path()).unwrap();
    let container = Container::create(&container_id, bundle.str_path());
    assert!(container.is_err(), "expect {:?} to be ok", container);
    assert_eq!(container.err().unwrap(), Error::ContainerAlreadyExists);
}
