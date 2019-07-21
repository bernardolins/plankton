use std::path::PathBuf;
use crate::Error;
use serde::Deserialize;
use serde::Serialize;
use nix::mount;
use failure::ResultExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MountPoint {
    source: Option<PathBuf>,
    destination: PathBuf,
    filesystem_type: Option<PathBuf>,
}

impl MountPoint {
    pub fn create(source: Option<&str>, destination: &str, filesystem_type: Option<&str>) -> MountPoint {
        MountPoint {
            source: source.map(|s| PathBuf::from(s)),
            destination: PathBuf::from(destination),
            filesystem_type: filesystem_type.map(|s| PathBuf::from(s)),
        }
    }

    pub fn mount(&self) -> Result<(), Error> {
        let source = self.source.clone();
        let destination = self.destination.clone();
        let filesystem_type = self.filesystem_type.clone();

        mount::mount(
            source.as_ref() as Option<&PathBuf>,
            &destination as &PathBuf,
            filesystem_type.as_ref() as Option<&PathBuf>,
            nix::mount::MsFlags::empty(),
            None as Option<&PathBuf>
        ).context(
            format!(
                "cannot mount {:?} on {:?} using filesystem {:?}",
                source.unwrap_or(PathBuf::from("")),
                destination,
                filesystem_type.unwrap_or(PathBuf::from(""))
            )
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_point_create_returns_mountpoint() {
        let mount_point = MountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"));

        assert_eq!(mount_point.source, Some(PathBuf::from("/tmp")));
        assert_eq!(mount_point.destination, PathBuf::from("/tmp"));
        assert_eq!(mount_point.filesystem_type, Some(PathBuf::from("tmpfs")));
    }

    #[test]
    fn mount_point_mount_returns_ok() {
        let mount_point = MountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"));
        let mount_result = mount_point.mount();

        assert!(mount_result.is_ok(), "expect {:?} to be ok", &mount_result);
    }

    #[test]
    fn mount_point_mount_returns_error_if_src_does_not_exist() {
        let mount_point = MountPoint::create(Some("/tmp"), "/invalid", Some("tmpfs"));
        let mount_result = mount_point.mount();

        assert!(mount_result.is_err(), "expect {:?} to be err", &mount_result);
    }

    #[test]
    fn mount_point_mount_returns_error_if_fs_type_is_invalid() {
        let mount_point = MountPoint::create(Some("/tmp"), "/tmp", Some("invalid"));
        let mount_result = mount_point.mount();

        assert!(mount_result.is_err(), "expect {:?} to be err", &mount_result);
    }
}
