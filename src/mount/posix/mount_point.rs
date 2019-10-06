use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use crate::Error;
use failure::ResultExt;
use nix::mount::MsFlags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosixMountPoint {
    pub source: Option<PathBuf>,
    pub destination: PathBuf,
    pub filesystem_type: Option<PathBuf>,
    pub options: Option<Vec<String>>,
}

impl PosixMountPoint {
    pub fn create(source: Option<&str>, destination: &str, filesystem_type: Option<&str>, options: Option<Vec<String>>) -> PosixMountPoint {
        PosixMountPoint {
            source: source.map(|s| PathBuf::from(s)),
            destination: PathBuf::from(destination),
            filesystem_type: filesystem_type.map(|s| PathBuf::from(s)),
            options: options,
        }
    }

    pub fn flags(&self) -> Result<MsFlags, Error> {
        let options = self.options.clone();
        let options_vec = options.unwrap_or(Vec::<String>::new());
        let mut ms_flags = MsFlags::empty();
        for option in options_vec {
            if !option.contains("=") {
                let ms_flag = PosixMountPoint::parse_flag(&option)?;
                ms_flags.insert(ms_flag);
            }
        }
        Ok(ms_flags)
    }

    pub fn data(&self) -> Option<PathBuf> {
        let options = self.options.clone();
        let options_vec = options.unwrap_or(Vec::<String>::new());
        let mut data_vec = Vec::new();
        for option in options_vec {
            if option.contains("=") {
                data_vec.push(option);
            }
        }
        if data_vec.is_empty() {
            return None;
        }
        let opts = data_vec.join(",");
        Some(PathBuf::from(opts))
    }

    fn parse_flag(flag: &str) -> Result<MsFlags, Error> {
        match flag {
            "ro" => Ok(MsFlags::MS_RDONLY),
            "rdonly" => Ok(MsFlags::MS_RDONLY),
            "nosuid" => Ok(MsFlags::MS_NOSUID),
            "nodev" => Ok(MsFlags::MS_NODEV),
            "noexec" => Ok(MsFlags::MS_NOEXEC),
            "synchronous" => Ok(MsFlags::MS_SYNCHRONOUS),
            "remount" => Ok(MsFlags::MS_REMOUNT),
            "mandlock" => Ok(MsFlags::MS_MANDLOCK),
            "dirsync" => Ok(MsFlags::MS_DIRSYNC),
            "noatime" => Ok(MsFlags::MS_NOATIME),
            "nodiratime" => Ok(MsFlags::MS_NODIRATIME),
            "bind" => Ok(MsFlags::MS_BIND),
            "move" => Ok(MsFlags::MS_MOVE),
            "rec" => Ok(MsFlags::MS_REC),
            "silent" => Ok(MsFlags::MS_SILENT),
            "posixacl" => Ok(MsFlags::MS_POSIXACL),
            "unbindable" => Ok(MsFlags::MS_UNBINDABLE),
            "private" => Ok(MsFlags::MS_PRIVATE),
            "slave" => Ok(MsFlags::MS_SLAVE),
            "shared" => Ok(MsFlags::MS_SHARED),
            "relatime" => Ok(MsFlags::MS_RELATIME),
            "kernmount" => Ok(MsFlags::MS_KERNMOUNT),
            "i_version" => Ok(MsFlags::MS_I_VERSION),
            "strictatime" => Ok(MsFlags::MS_STRICTATIME),
            "active" => Ok(MsFlags::MS_ACTIVE),
            "nouser" => Ok(MsFlags::MS_NOUSER),
            "rmt_mask" => Ok(MsFlags::MS_RMT_MASK),
            "mgc_val" => Ok(MsFlags::MS_MGC_VAL),
            "mgc_msk" => Ok(MsFlags::MS_MGC_MSK),
            _ => Err(Error::from("unknown mount flag".to_string())).context(flag.to_string())?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::mount::MsFlags;

    #[test]
    fn create_returns_mountpoint() {
        let mount_point = PosixMountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"), None);
        assert_eq!(mount_point.source, Some(PathBuf::from("/tmp")));
        assert_eq!(mount_point.destination, PathBuf::from("/tmp"));
        assert_eq!(mount_point.filesystem_type, Some(PathBuf::from("tmpfs")));
    }

    #[test]
    fn flags_return_parsed_flags() {
        let mount_point = PosixMountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"), Some(vec!["ro".to_string(), "rec".to_string()]));
        assert!(mount_point.flags().is_ok());
        assert!(mount_point.flags().unwrap().contains(MsFlags::MS_RDONLY));
        assert!(mount_point.flags().unwrap().contains(MsFlags::MS_REC));
    }

    #[test]
    fn flags_return_error_when_flag_is_invalid() {
        let mount_point = PosixMountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"), Some(vec!["invalid".to_string()]));
        assert!(mount_point.flags().is_err());
    }

    #[test]
    fn data_return_parsed_data() {
        let mount_point = PosixMountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"), Some(vec!["mode=755".to_string(), "size=65536k".to_string()]));
        assert!(mount_point.data().is_some());
        assert_eq!(mount_point.data(), Some(PathBuf::from("mode=755,size=65536k")));
    }

    #[test]
    fn data_return_none_with_no_data() {
        let mount_point = PosixMountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"), Some(vec!["ro".to_string()]));
        assert!(mount_point.data().is_none());
        assert_eq!(mount_point.data(), None);
    }

    #[test]
    fn data_return_none_with_no_options() {
        let mount_point = PosixMountPoint::create(Some("/tmp"), "/tmp", Some("tmpfs"), None);
        assert!(mount_point.data().is_none());
        assert_eq!(mount_point.data(), None);
    }
}
