use nix::sched;
use std::process;
use nix::sched::CloneFlags;

#[derive(Debug)]
pub enum Namespace {
    PID,
    UTS,
    IPC,
    USER,
    MOUNT,
    CGROUP,
    NETWORK,
}

impl Namespace {
    pub fn join_new(&self) {
        let mut flags = CloneFlags::empty();

        match *self {
            Namespace::PID => flags.insert(CloneFlags::CLONE_NEWPID),
            Namespace::UTS => flags.insert(CloneFlags::CLONE_NEWUTS),
            Namespace::IPC => flags.insert(CloneFlags::CLONE_NEWIPC),
            Namespace::USER => flags.insert(CloneFlags::CLONE_NEWUSER),
            Namespace::MOUNT => flags.insert(CloneFlags::CLONE_NEWNS),
            Namespace::CGROUP => flags.insert(CloneFlags::CLONE_NEWCGROUP),
            Namespace::NETWORK => flags.insert(CloneFlags::CLONE_NEWNET),
        }

        if let Err(err) = sched::unshare(flags) {
            eprintln!("could not create {:?} namespace: {}", self, err);
            process::exit(-4);
        }
    }
}

