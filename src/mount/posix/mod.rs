mod mount_point;

use crate::Error;
use crate::mount::Mounts;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use failure::ResultExt;
use nix::mount;
use nix::mount::MsFlags;
use serde::Deserialize;
use serde::Serialize;
use self::mount_point::PosixMountPoint;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosixMounts {
    mounts: Vec<PosixMountPoint>,
}

impl FromSpec<PosixSpec> for PosixMounts {
    fn from_spec(spec: PosixSpec) -> Result<PosixMounts, Error> {
        let mut mounts = Vec::<PosixMountPoint>::new();
        for mount in spec.mounts() {
            mounts.push(
                PosixMountPoint::create(mount.source(), mount.destination(), mount.filesystem_type(), mount.options())
            );
        }
        Ok(PosixMounts{ mounts })
    }
}

impl Mounts for PosixMounts {
    fn mount_all(&self) -> Result<(), Error> {
        for mount in self.mounts.clone() {
            mount::mount(
                mount.source.as_ref() as Option<&PathBuf>,
                &mount.destination as &PathBuf,
                mount.filesystem_type.as_ref() as Option<&PathBuf>,
                mount.flags()? as MsFlags,
                mount.data().as_ref() as Option<&PathBuf>
            ).context(mount_error_message(mount))?;
        }
        Ok(())
    }
}

fn mount_error_message(mp: PosixMountPoint) -> String {
    format!("cannot mount {:?} on {:?} using filesystem {:?}",
            mp.source.unwrap_or(PathBuf::from("")), mp.destination, mp.filesystem_type.unwrap_or(PathBuf::from(""))
    )
}
