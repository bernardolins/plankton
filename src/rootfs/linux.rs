use crate::Error;
use crate::rootfs::RootFS;
use crate::spec::FromSpec;
use crate::spec::PosixSpec;
use failure::ResultExt;
use nix::unistd;
use std::path::PathBuf;

pub struct LinuxRootFS {
    path: PathBuf,
    ro: bool,
}

impl RootFS for LinuxRootFS {
    fn set(&self, bundle_path: PathBuf) -> Result<(), Error> {
        let root_path = bundle_path.join(&self.path);
        if !root_path.is_dir() {
            Err(Error::from("no such directory".to_string())).context(format!("{:?}", &root_path))?;
        }
        unistd::chroot(&root_path).context(format!("{:?}", &root_path))?;
        Ok(())
    }
}

impl FromSpec<PosixSpec> for LinuxRootFS {
    fn from_spec(spec: PosixSpec) -> Result<LinuxRootFS, Error> {
        let rootfs = LinuxRootFS {
            path: PathBuf::from(spec.root().path()),
            ro: spec.root().readonly(),
        };
        Ok(rootfs)
    }
}
