use std::ptr;
use std::process;
use std::process::Command;
use std::io::Error;

use nix::sched;
use nix::sched::CloneFlags;

use nix::unistd;

fn child(spec: &::oci::Spec) -> isize {
    unistd::sethostname(spec.hostname);
    return 0;
}

pub fn run_container(spec: ::oci::Spec) {
    let stack = &mut[0; 1024*1024];

    let exec_fn = Box::new(|| child(&spec));

    let p = sched::clone(exec_fn, stack, CloneFlags::CLONE_NEWUTS, None);
    let p = p.unwrap();
}
