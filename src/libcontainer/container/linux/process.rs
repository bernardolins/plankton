use std::env;
use std::process;
use std::ffi::CString;
use std::path::PathBuf;

use nix::sched;
use nix::unistd;
use nix::sys::wait;
use nix::unistd::Pid;
use nix::sched::CloneFlags;
use nix::sys::wait::WaitPidFlag;

use crate::libcontainer::environment::Environment;

pub fn create(environment: &Environment) -> i32 {
    let stack = &mut[0; 1024*1024];
    let exec_fn = Box::new(|| child(&environment));

    match sched::clone(exec_fn, stack, CloneFlags::empty(), None) {
        Ok(pid) => pid.as_raw(),
        Err(err) => {
            eprintln!("clone error: {}", err);
            process::exit(-1);
        }
    }
}

pub fn wait(pid: i32) {
    match wait::waitpid(Pid::from_raw(pid), Some(WaitPidFlag::__WALL)) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("wait error: {}", err);
            process::exit(-1);
        }
    }
}

fn child(environment: &Environment) -> isize {
    try_set_working_dir(environment.working_dir());
    try_exec(environment.argv());

    return 0;
}

fn try_set_working_dir(working_dir: &PathBuf) {
    match env::set_current_dir(working_dir) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("error setting container working dir: {}", err);
            process::exit(-1);
        }
    }
}

fn try_exec(argv: &Vec<String>) {
    let args: Vec<CString> = argv.iter().map(|arg|
        try_turn_to_cstring(&arg)
    ).collect();

    let path = args[0].clone();

    match unistd::execvp(&path, &args) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("exec error: {}", err);
            process::exit(-1);
        }
    }
}

fn try_turn_to_cstring(arg: &str) -> CString {
    match CString::new(arg.to_string()) {
        Ok(cstring) => cstring,
        Err(err) => {
            eprintln!("error parsing arg: {}", err);
            process::exit(-1);
        }
    }
}
