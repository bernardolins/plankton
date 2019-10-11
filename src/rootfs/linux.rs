use crate::Error;
use crate::spec::FromSpec;
use crate::spec::PosixSpec;
use std::path::PathBuf;

pub struct LinuxRootFS {
    path: PathBuf,
    ro: bool,
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
