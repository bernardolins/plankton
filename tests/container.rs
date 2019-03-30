extern crate cr7;
extern crate rand;

mod common;

use cr7::container;
use cr7::error::Error;

use common::{TestBundle, ConfigTemplate};

#[test]
fn container_create() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let bundle_path = test_bundle.str_path();

    let container_id = format!("container-{}", rand::random::<u32>());
    let container = container::create(&container_id, bundle_path);

    assert!(container.is_ok(), "expect {:?} to be ok", container);
}

#[test]
fn container_state() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let bundle_path = test_bundle.str_path();
    let container_id = format!("container-{}", rand::random::<u32>());

    container::create(&container_id, bundle_path);
    let state = container::state(&container_id);

    assert!(state.is_ok(), "expect {} to be ok", &state.unwrap());
}

#[test]
fn container_already_exist() {
    let test_bundle1 = TestBundle::new(ConfigTemplate::Valid);
    let test_bundle2 = TestBundle::new(ConfigTemplate::Valid);
    let bundle_path1 = test_bundle1.str_path();
    let bundle_path2 = test_bundle2.str_path();
    let container_id = format!("container-{}", rand::random::<u32>());

    container::create(&container_id, bundle_path1).unwrap();
    let container = container::create(&container_id, bundle_path2);

    assert!(container.is_err(), "expect {:?} to be ok", container);
    assert_eq!(container.err().unwrap(), Error::ContainerAlreadyExists);
}
