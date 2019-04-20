use std::env;
use std::process;
use std::error::Error;
use std::ffi::CString;
use std::path::PathBuf;

use nix::sched;
use nix::unistd;
use nix::sys::wait;
use nix::unistd::Pid;
use nix::sched::CloneFlags;
use nix::sys::wait::WaitPidFlag;

use crate::libcontainer::linux::environment::Environment;

enum ExitCode {
    Create,
    Wait,
    Rootfs,
    SetWorkingDir,
    SetHostname,
    Exec,
}

pub fn create(environment: &Environment) -> i32 {
    let stack = &mut[0; 1024*1024];
    let exec_fn = Box::new(|| child(&environment));

    match sched::clone(exec_fn, stack, CloneFlags::empty(), None) {
        Ok(pid) => pid.as_raw(),
        Err(err) => {
            exit("clone error", ExitCode::Create, Box::new(err));
            return 0;
        }
    }
}

pub fn wait(pid: i32) {
    if let Err(err) = wait::waitpid(Pid::from_raw(pid), Some(WaitPidFlag::__WALL)) {
        exit("wait error", ExitCode::Wait, Box::new(err));
    }
}

fn child(environment: &Environment) -> isize {
    try_set_chroot(environment.rootfs());
    try_set_working_dir(environment.working_dir());
    try_set_hostname(environment.hostname());

    try_exec(environment.argv());

    return 0;
}

fn try_set_chroot(rootfs: &PathBuf) {
    if let Err(err) = unistd::chroot(rootfs) {
        exit("error setting container root", ExitCode::Rootfs, Box::new(err));
    }
}

fn try_set_working_dir(working_dir: &PathBuf) {
    if let Err(err) = env::set_current_dir(working_dir) {
       exit("error setting container working dir", ExitCode::SetWorkingDir, Box::new(err));
    }
}

fn try_exec(argv: &Vec<String>) {
    let args: Vec<CString> = argv.iter().map(|arg|
        CString::new(arg.to_string()).expect("error parsing argument")
    ).collect();
    let path = args[0].clone();

    if let Err(err) = unistd::execvp(&path, &args) {
        exit("exec error", ExitCode::Exec, Box::new(err));
    }
}

fn try_set_hostname(option_hostname: &Option<String>) {
    if let Some(hostname) = option_hostname {
        if let Err(err) = unistd::sethostname(hostname) {
           exit("error setting container hostname", ExitCode::SetHostname, Box::new(err));
        }
    }
}

fn exit(message: &str, code: ExitCode, err: Box<Error>) {
    eprintln!("{}: {}", message, err);
    process::exit(code as i32);
}
