use nix::sched;
use nix::unistd;
use nix::sched::CloneFlags;
use nix::sys::wait;
use std::ffi;

fn child(spec: &::oci::Spec) -> isize {
    unistd::sethostname(&spec.hostname).expect("sethostname failed");

    let pid = unistd::getpid();

    println!("{}", pid);

    let bin = spec.process.args[0].clone();
    let args = spec.process.args.clone();

    unistd::execvp(&bin, &args).unwrap();

    return 0;
}

pub fn run_container(spec: ::oci::Spec) {
    let stack = &mut[0; 1024*1024];

    let exec_fn = Box::new(|| child(&spec));

    let mut flags = CloneFlags::empty();

    flags.insert(CloneFlags::CLONE_NEWUTS);
    flags.insert(CloneFlags::CLONE_NEWPID);

    if let Ok(pid) = sched::clone(exec_fn, stack, flags, None) {
        wait::waitpid(pid, Some(wait::WaitPidFlag::__WALL)).unwrap();
    }
}
