extern crate cr7;
extern crate rand;
extern crate serde_json;

mod common;

use std::io::BufReader;

use cr7::libcontainer::config::root::Root;

use common::ConfigTemplate;

use serde_json::error::Error;

fn build_root(template: ConfigTemplate) -> Result<Root, Error> {
    let template_file = template.file();
    let reader = BufReader::new(template_file);
    serde_json::from_reader(reader)
}

#[test]
fn config_root_without_path() {
    let root = build_root(ConfigTemplate::RootNoPath);
    assert!(root.is_err(), "expect {:?} to be err", &root);
}

#[test]
fn config_root_without_readonly() {
    let root = build_root(ConfigTemplate::RootNoReadonly);
    assert!(root.is_ok(), "expect {:?} to be ok", &root);
}

#[test]
fn config_root_valid() {
    let root = build_root(ConfigTemplate::RootValid);
    assert!(root.is_ok(), "expect {:?} to be ok", &root);
}

#[test]
fn root_public_method_path() {
    let root = build_root(ConfigTemplate::RootValid);
    assert!(root.is_ok(), "expect {:?} to be ok", &root);
    assert_eq!(root.unwrap().path(), "rootfs");
}

#[test]
fn root_public_method_readonly() {
    let root = build_root(ConfigTemplate::RootValid);
    assert!(root.is_ok(), "expect {:?} to be ok", &root);
    assert_eq!(root.unwrap().readonly(), true);
}
