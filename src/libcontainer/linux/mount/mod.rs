pub mod error;

use nix::mount;

use self::error::Error;

pub struct MountPoint {
    source: Option<&'static str>,
    destination: &'static str,
    filesystem_type: Option<&'static str>,
}

impl MountPoint {
    pub fn create(source: Option<&'static str>, destination: &'static str, filesystem_type: Option<&'static str>) -> MountPoint {
        MountPoint {
            source: source,
            destination: destination,
            filesystem_type: filesystem_type,
        }
    }

    pub fn mount(&self) -> Result<(), Error> {
        let mount_result = mount::mount(
            self.source as Option<&'static str>,
            self.destination as &'static str,
            self.filesystem_type as Option<&'static str>,
            nix::mount::MsFlags::empty(),
            None as Option<&'static str>
        );

        if let Err(err) = mount_result {
            Err(Error::new(self.source, self.destination, self.filesystem_type, &format!("{}", err)))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_point_create_returns_mountpoint() {
        let mount_point = MountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"));

        assert_eq!(mount_point.source, Some("/tmp"));
        assert_eq!(mount_point.destination, "/tmp");
        assert_eq!(mount_point.filesystem_type, Some("tmpfs"));
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
