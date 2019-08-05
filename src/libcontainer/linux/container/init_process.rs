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

use crate::Error;
use crate::filesystem::pathbuf;
use crate::libcontainer::linux::rlimit::Rlimit;
use crate::libcontainer::linux::mount::MountPoint;
use crate::libcontainer::linux::container::Container;

use failure::ResultExt;

pub fn create(container: &Container) -> Result<i32, Error> {
    let stack = &mut[0; 1024*1024];
    let exec_fn = Box::new(|| child(&container));

    let pid = sched::clone(exec_fn, stack, CloneFlags::empty(), None)?;
    Ok(pid.as_raw())
}

pub fn wait(pid: i32) -> Result<(), Error> {
    wait::waitpid(Pid::from_raw(pid), Some(WaitPidFlag::__WALL))?;
    Ok(())
}

fn child(container: &Container) -> isize {
    if let Err(err) = try_create_container(container) {
        eprintln!("Error: {}", err);
        process::exit(exitcode::OSERR);
    }

    return 0;
}

fn try_create_container(container: &Container) -> Result<(), Error> {
    try_set_chroot(container.environment.rootfs())?;
    try_set_env_vars(container.environment.env_vars());
    try_set_mount_points(container.environment.mount_list())?;
    try_set_working_dir(container.environment.working_dir())?;
    try_set_hostname(container.environment.hostname())?;
    try_set_rlimits(container.environment.rlimits())?;
    try_exec(container.environment.argv())?;
    Ok(())
}

fn try_set_chroot(rootfs: &PathBuf) -> Result<(), Error> {
    unistd::chroot(rootfs).context(pathbuf::to_string(rootfs.to_path_buf()))?;
    Ok(())
}

fn try_set_env_vars(env_vars: &Vec<(String, String)>) {
    for (key, val) in env_vars {
        let k = OsStr::new(key);
        let v = OsStr::new(val);

        env::set_var(k, v);
    }
}

fn try_set_mount_points(mount_list: &Vec<MountPoint>) -> Result<(), Error> {
    for mount_point in mount_list {
        mount_point.mount()?;
    }
    Ok(())
}

fn try_set_working_dir(working_dir: &PathBuf) -> Result<(), Error> {
    env::set_current_dir(working_dir).context(pathbuf::to_string(working_dir.to_path_buf()))?;
    Ok(())
}

fn try_set_hostname(option_hostname: &Option<String>) -> Result<(), Error> {
    if let Some(hostname) = option_hostname {
        unistd::sethostname(hostname).context(hostname.to_string())?;
    }
    Ok(())
}

fn try_set_rlimits(rlimits: &Vec<Rlimit>) -> Result<(), Error> {
    for rlimit in rlimits {
        rlimit.set()?;
    }
    Ok(())
}

fn try_exec(argv: &Vec<String>) -> Result<(), Error> {
    let args: Vec<CString> = argv.iter().map(|arg|
        CString::new(arg.to_string()).expect("error parsing argument")
    ).collect();
    let path = args[0].clone();

    unistd::execvp(&path, &args).context(format!("{:?}", &argv))?;
    Ok(())
}
