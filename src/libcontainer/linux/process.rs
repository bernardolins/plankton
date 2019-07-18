use std::env;
use std::process;
use std::ffi::OsStr;
use std::ffi::CString;
use std::path::PathBuf;

use nix::sched;
use nix::unistd;
use nix::sys::wait;
use nix::unistd::Pid;
use nix::sched::CloneFlags;
use nix::sys::wait::WaitPidFlag;

use crate::libcontainer::Error;
use crate::libcontainer::linux::rlimit::Rlimit;
use crate::libcontainer::linux::mount::MountPoint;
use crate::libcontainer::linux::environment::Environment;

enum ExitCode {
    Rootfs,
    Mount,
    SetWorkingDir,
    SetHostname,
    Rlimit,
    Exec,
}

pub fn create(environment: &Environment) -> Result<i32, Error> {
    let stack = &mut[0; 1024*1024];
    let exec_fn = Box::new(|| child(&environment));

    let pid = sched::clone(exec_fn, stack, CloneFlags::empty(), None)?;
    Ok(pid.as_raw())
}

pub fn wait(pid: i32) -> Result<(), Error> {
    wait::waitpid(Pid::from_raw(pid), Some(WaitPidFlag::__WALL))?;
    Ok(())
}

fn child(environment: &Environment) -> isize {
    try_set_chroot(environment.rootfs());
    try_set_env_vars(environment.env_vars());
    try_set_mount_points(environment.mount_list());
    try_set_working_dir(environment.working_dir());
    try_set_hostname(environment.hostname());
    try_set_rlimits(environment.rlimits());

    try_exec(environment.argv());

    return 0;
}

fn try_set_env_vars(env_vars: &Vec<(String, String)>) {
    for (key, val) in env_vars {
        let k = OsStr::new(key);
        let v = OsStr::new(val);

        env::set_var(k, v);
    }
}

fn try_set_chroot(rootfs: &PathBuf) {
    if let Err(err) = unistd::chroot(rootfs) {
        exit(ExitCode::Rootfs, Box::new(Error::from(err)));
    }
}

fn try_set_mount_points(mount_list: &Vec<MountPoint>) {
    for mount_point in mount_list {
        if let Err(err) = mount_point.mount() {
            exit(ExitCode::Mount, Box::new(Error::from(format!("{}", err))));
        }
    }
}

fn try_set_working_dir(working_dir: &PathBuf) {
    if let Err(err) = env::set_current_dir(working_dir) {
       exit(ExitCode::SetWorkingDir, Box::new(Error::from(format!("{}", err))));
    }
}

fn try_set_rlimits(rlimits: &Vec<Rlimit>) {
    for rlimit in rlimits {
        if let Err(err) = rlimit.set() {
            exit(ExitCode::Rlimit, Box::new(Error::from(err)));
        }
    }
}

fn try_exec(argv: &Vec<String>) {
    let args: Vec<CString> = argv.iter().map(|arg|
        CString::new(arg.to_string()).expect("error parsing argument")
    ).collect();
    let path = args[0].clone();

    if let Err(err) = unistd::execvp(&path, &args) {
        exit(ExitCode::Exec, Box::new(Error::from(err)));
    }
}

fn try_set_hostname(option_hostname: &Option<String>) {
    if let Some(hostname) = option_hostname {
        if let Err(err) = unistd::sethostname(hostname) {
           exit(ExitCode::SetHostname, Box::new(Error::from(err)));
        }
    }
}

fn exit(code: ExitCode, err: Box<Error>) {
    eprintln!("{}", err);
    process::exit(code as i32);
}
