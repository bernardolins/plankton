use crate::Error;
use crate::filesystem::pathbuf;
use crate::libcontainer::linux::rlimit::Rlimit;
use crate::libcontainer::linux::user::User;
use crate::libcontainer::linux::mount::MountPoint;
use failure::ResultExt;
use nix::sched;
use nix::unistd;
use nix::sys::wait;
use nix::unistd::Pid;
use nix::unistd::Uid;
use nix::unistd::Gid;
use nix::sched::CloneFlags;
use nix::sys::wait::WaitPidFlag;
use std::env;
use std::process;
use std::ffi::OsStr;
use std::ffi::CString;
use std::path::PathBuf;
use super::Environment;

pub fn clone(environment: &Environment) -> Result<i32, Error> {
    let stack = &mut[0; 1024*1024];
    let exec_fn = Box::new(|| child_callback(&environment));

    let pid = sched::clone(exec_fn, stack, CloneFlags::empty(), None)?;
    Ok(pid.as_raw())
}

pub fn wait(pid: i32) -> Result<(), Error> {
    wait::waitpid(Pid::from_raw(pid), Some(WaitPidFlag::__WALL))?;
    Ok(())
}

pub fn child_callback(environment: &Environment) -> isize {
    if let Err(err) = try_create_environment(environment) {
        eprintln!("Error: {}", err);
        process::exit(exitcode::OSERR);
    }
    return 0;
}

fn try_create_environment(environment: &Environment) -> Result<(), Error> {
    apply_chroot(&environment.rootfs)?;
    apply_env_vars(&environment.env_vars);
    apply_mount_points(&environment.mount_list)?;
    apply_working_dir(&environment.working_dir)?;
    apply_hostname(&environment.hostname)?;
    apply_rlimits(&environment.rlimits)?;
    apply_user(&environment.user)?;
    try_exec(&environment.argv)?;
    Ok(())
}

fn apply_chroot(rootfs: &PathBuf) -> Result<(), Error> {
    unistd::chroot(rootfs).context(pathbuf::to_string(rootfs.to_path_buf()))?;
    Ok(())
}

fn apply_env_vars(env_vars: &Vec<(String, String)>) {
    for (key, val) in env_vars {
        let k = OsStr::new(key);
        let v = OsStr::new(val);
        env::set_var(k, v);
    }
}

fn apply_mount_points(mount_list: &Vec<MountPoint>) -> Result<(), Error> {
    for mount_point in mount_list {
        mount_point.mount()?;
    }
    Ok(())
}

fn apply_working_dir(working_dir: &PathBuf) -> Result<(), Error> {
    env::set_current_dir(working_dir).context(pathbuf::to_string(working_dir.to_path_buf()))?;
    Ok(())
}

fn apply_hostname(option_hostname: &Option<String>) -> Result<(), Error> {
    if let Some(hostname) = option_hostname {
        unistd::sethostname(hostname).context(hostname.to_string())?;
    }
    Ok(())
}

fn apply_rlimits(rlimits: &Vec<Rlimit>) -> Result<(), Error> {
    for rlimit in rlimits {
        rlimit.set()?;
    }
    Ok(())
}

fn apply_user(user: &User) -> Result<(), Error> {
    let uid = Uid::from_raw(user.uid() as u32);
    unistd::setuid(uid).context(format!("cannot set user id {}", uid))?;

    let gid = Gid::from_raw(user.gid() as u32);
    unistd::setgid(gid).context(format!("cannot set group id {}", gid))?;

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
