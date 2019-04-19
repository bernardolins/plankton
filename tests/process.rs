extern crate cr7;
extern crate serde_json;

mod common;

use std::io::BufReader;

use cr7::libcontainer::config::process::Process;

use common::ConfigTemplate;

use serde_json::error::Error;

fn build_process(template: ConfigTemplate) -> Result<Process, Error> {
    let template_file = template.file();
    let reader = BufReader::new(template_file);
    serde_json::from_reader(reader)
}

#[test]
fn config_process_without_args() {
    let process = build_process(ConfigTemplate::ProcessNoArgs);
    assert!(process.is_err(), "expect {:?} to be err", &process);
}

#[test]
fn config_process_valid() {
    let process = build_process(ConfigTemplate::ProcessValid);
    assert!(process.is_ok(), "expect {:?} to be ok", &process);
}

#[test]
fn process_public_method_args() {
    let process = build_process(ConfigTemplate::ProcessValid);
    assert_eq!(process.unwrap().args(), &["sh"]);
}
