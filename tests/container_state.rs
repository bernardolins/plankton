extern crate cr7;
extern crate rand;

mod common;

use cr7::container;

use common::{TestBundle, ConfigTemplate};

#[test]
fn container_state() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let bundle_path = test_bundle.str_path();
    let container_id = format!("container-{}", rand::random::<u32>());

    container::operations::create(&container_id, bundle_path).unwrap();
    let state = container::operations::state(&container_id);

    assert!(state.is_ok(), "expect {} to be ok", &state.unwrap());
}
