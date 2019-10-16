use crate::Error;
use crate::bundle::Bundle;
use crate::container::ContainerTrait;
use crate::spec::FromSpec;
use crate::spec::PosixSpec;
use crate::process::Process;
use crate::process::PosixProcess;
use crate::mount::Mounts;
use crate::mount::PosixMounts;
use crate::rootfs::RootFS;
use crate::rootfs::LinuxRootFS;
use crate::platform::linux::Namespaces;
use failure::ResultExt;
use std::path::PathBuf;

pub struct LinuxContainer;

impl ContainerTrait for LinuxContainer {
    fn create(_id: &str, bundle_dir: &str) -> Result<(), Error> {
        let bundle = Bundle::<PosixSpec>::open(bundle_dir)?;
        let spec = bundle.load_config()?;
        let path = PathBuf::from(bundle_dir);
        let mut process = PosixProcess::from_spec(spec.clone())?;
        process.before_exec(move || {
            let rootfs = LinuxRootFS::from_spec(spec.clone())?;
            let mounts = PosixMounts::from_spec(spec.clone())?;
            let namespaces = Namespaces::from_spec(spec.clone())?;
            rootfs.set(path.clone())?;
            mounts.mount_all()?;
            Ok(())
        });
        let mut child = process.spawn()?;
        child.wait().context(format!("error waiting process {:?}", child.id()))?;
        Ok(())
    }
}
