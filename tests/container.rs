extern crate cr7;
extern crate rand;

mod common;

use cr7::bundle::Bundle;
use cr7::container::Container;
use cr7::error::Error;

use common::{TestBundle, ConfigTemplate};

#[test]
fn create_container() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let bundle = Bundle::new(&test_bundle.str_path()).expect("failed to create bundle");

    let container_id = format!("container-{}", rand::random::<u32>());
    let container = Container::create(&container_id, bundle);
    assert!(container.is_ok(), "expect {:?} to be ok", container);
}

#[test]
fn container_already_exist() {
    let test_bundle1 = TestBundle::new(ConfigTemplate::Valid);
    let test_bundle2 = TestBundle::new(ConfigTemplate::Valid);

    let bundle1 = Bundle::new(&test_bundle1.str_path()).expect("failed to create bundle");
    let bundle2 = Bundle::new(&test_bundle2.str_path()).expect("failed to create bundle");

    let container_id = format!("container-{}", rand::random::<u32>());
    Container::create(&container_id, bundle1).unwrap();
    let container = Container::create(&container_id, bundle2);
    assert!(container.is_err(), "expect {:?} to be ok", container);
    assert_eq!(container.err().unwrap(), Error::ContainerAlreadyExists);
}

#[test]
fn container_state() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let bundle = Bundle::new(&test_bundle.str_path()).expect("failed to create bundle");

    let container_id = format!("container-{}", rand::random::<u32>());
    let container = Container::create(&container_id, bundle);
    let state = container.unwrap().state();
    assert_eq!(state.oci_version, "1.0.1-dev");
    assert_eq!(state.id, container_id);
    assert_eq!(state.status, "creating");
}
