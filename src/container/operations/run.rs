use crate::bundle::Bundle;
use crate::config;
use crate::container::Container;
use crate::container::Status;
use crate::container::environment;
use crate::error::Error;
use crate::config::process::Process;

use nix::sched;
use nix::unistd;
use nix::sched::CloneFlags;
use nix::sys::wait;

use std::ffi::CString;

pub fn run(container_id: &str, bundle_path: &str) -> Result<(), Error> {
    let container = environment::load(container_id)?;

    if let Some(_) = container {
        return Err(Error::ContainerAlreadyExists)
    }

    let bundle = Bundle::new(bundle_path)?;
    let config = config::load(&bundle.config_path())?;

    let mut container = Container::new(container_id, bundle_path, config)?;

    environment::save(&container)?;

    let pid = init(&container)?;

    /*
     * Mark container as running
     */
    container.set_pid(Some(pid));
    container.set_status(Status::Running);
    environment::save(&container)?;

    wait::waitpid(unistd::Pid::from_raw(pid), Some(wait::WaitPidFlag::__WALL))?;

    /*
     * Mark container as stopped
     */
    container.set_pid(None);
    container.set_status(Status::Stopped);
    environment::save(&container)?;

    Ok(())
}

fn init(container: &Container) -> Result<i32, Error> {
    let stack = &mut[0; 1024*1024];
    let process_config = container.config.process();
    let callback = Box::new(|| clone_callback(&process_config));

    let flags = CloneFlags::empty();
    let pid = sched::clone(callback, stack, flags, None)?;

    Ok(pid.as_raw())
}

fn clone_callback(process_config: &Process) -> isize {
    let args = process_config.args();
    let cstring_args: Vec<CString> = args.iter().map(|arg| CString::new(arg.to_string()).unwrap()).collect();
    let bin = cstring_args[0].clone();

    unistd::execvp(&bin, &cstring_args[..]).unwrap();

    return 0
}
