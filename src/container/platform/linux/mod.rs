use crate::Error;
use crate::bundle::Bundle;
use crate::container::ContainerInfo;
use crate::container::ContainerBuilder;
use crate::container::ContainerRunner;
use crate::container::State;
use crate::container::Status;
use crate::spec::FromSpec;
use crate::spec::PosixSpec;
use crate::process::Process;
use crate::process::PosixProcess;
use crate::mount::Mounts;
use crate::mount::PosixMounts;
use crate::rootfs::RootFS;
use crate::rootfs::LinuxRootFS;
use crate::linux::Namespaces;
use failure::ResultExt;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;


const SAVED_STATE_DIR: &str = "/run/plankton";

pub struct Linux {
    id: String,
    namespaces: Namespaces,
    entrypoint: PosixProcess,
    bundle_path: PathBuf,
}

impl ContainerRunner for Linux {
    fn run_entrypoint(&mut self) -> Result<(), Error> {
        self.namespaces.enter()?;
        let mut child = self.entrypoint.spawn()?;
        child.wait().context(format!("error waiting process {:?}", child.id()))?;
        Ok(())
    }
}

impl ContainerBuilder for Linux {
    type Spec = PosixSpec;

    fn from_bundle(id: &str, bundle: Bundle<Self::Spec>) -> Result<Linux, Error> {
        let id = String::from(id);
        let spec = bundle.load_config()?;

        let namespaces = Namespaces::from_spec(spec.clone())?;
        let mut entrypoint = PosixProcess::from_spec(spec.clone())?;

        let path = bundle.path();
        entrypoint.before_exec(move || {
            let rootfs = LinuxRootFS::from_spec(spec.clone())?;
            let mounts = PosixMounts::from_spec(spec.clone())?;
            rootfs.set(path.clone())?;
            mounts.mount_all()?;
            Ok(())
        });

        let bundle_path = bundle.path();
        Ok(Linux { id, bundle_path, namespaces, entrypoint })
    }
}

impl ContainerInfo for Linux {
    fn exists(id: &str) -> bool {
        let path = PathBuf::from(format!("{}/{}.json", SAVED_STATE_DIR, id));
        path.is_file()
    }

    fn current_state(id: &str) -> Result<State, Error> {
        let path = PathBuf::from(format!("{}/{}.json", SAVED_STATE_DIR, id));
        let file = File::open(path).context("cannot open container state file".to_string())?;
        let reader = BufReader::new(file);
        let state: State = serde_json::from_reader(reader).context("error loading container state".to_string())?;
        Ok(state)
    }

    fn update_state(id: &str, new_state: State) -> Result<(), Error> {
        let path = PathBuf::from(format!("{}/{}.json", SAVED_STATE_DIR, id));
        let json = serde_json::to_string(&new_state).context("cannot save container state".to_string())?;
        fs::write(&path, json).context(format!("cannot save container state to file {:?}", &path))?;
        Ok(())
    }
}
