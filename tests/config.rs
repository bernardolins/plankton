extern crate cr7;
extern crate rand;

mod common;

use cr7::bundle::Bundle;
use cr7::config;
use cr7::error::Error;

use common::{TestBundle, ConfigTemplate};

#[test]
fn config_file_synxtax_error() {
    let test_bundle = TestBundle::new(ConfigTemplate::SyntaxError);
    let bundle = Bundle::new(test_bundle.str_path()).unwrap();
    let config = config::load(bundle.config_path());
    assert!(config.is_err(), "expect {:?} to be err", config);
    assert_eq!(config.err().unwrap(), Error::ConfigSyntax);
}

#[test]
fn config_file_invalid() {
    let test_bundle = TestBundle::new(ConfigTemplate::Invalid);
    let bundle = Bundle::new(test_bundle.str_path()).unwrap();
    let config = config::load(bundle.config_path());
    assert!(config.is_err(), "expect {:?} to be err", config);
    assert_eq!(config.err().unwrap(), Error::ParseConfig);
}
