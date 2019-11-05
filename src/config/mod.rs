mod mount;
mod process;
mod root;

use crate::spec::Spec;
use serde::Serialize;
use serde::Deserialize;

pub use self::mount::Mount;
pub use self::process::Process;
pub use self::root::Root;

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    mounts: Option<Vec<Mount>>,
    process: Option<Process>,
    root: Option<Root>,
}

impl Spec for ConfigFile {
    type Mount = Mount;
    type Root = Root;
    type Process = Process;

    fn get_root(&self) -> Option<&Self::Root> {
        self.root.as_ref()
    }

    fn get_root_clone(&self) -> Option<Self::Root> {
        self.get_root().cloned()
    }

    fn get_mounts(&self) -> Option<&Vec<Self::Mount>> {
        self.mounts.as_ref()
    }

    fn get_mounts_clone(&self) -> Option<Vec<Self::Mount>> {
        self.get_mounts().cloned()
    }

    fn get_process(&self) -> Option<&Self::Process> {
        self.process.as_ref()
    }

    fn get_process_clone(&self) -> Option<Self::Process> {
        self.get_process().cloned()
    }

}
