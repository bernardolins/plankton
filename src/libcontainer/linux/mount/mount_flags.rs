use crate::Error;
use failure::ResultExt;
use nix::mount::MsFlags;
use std::path::PathBuf;

pub fn extract_flags(options: Option<Vec<String>>) -> Result<MsFlags, Error> {
    let options_vec = options.unwrap_or(Vec::<String>::new());
    let mut ms_flags = MsFlags::empty();
    for option in options_vec {
        if !option.contains("=") {
            let ms_flag = parse_flag(&option)?;
            ms_flags.insert(ms_flag);
        }
    }
    Ok(ms_flags)
}

pub fn extract_data(options: Option<Vec<String>>) -> Result<Option<PathBuf>, Error> {
    let options_vec = options.unwrap_or(Vec::<String>::new());
    let mut data_vec = Vec::new();
    for option in options_vec {
        if option.contains("=") {
            data_vec.push(option);
        }
    }
    let opts = data_vec.join(",");
    let path = PathBuf::from(opts);
    Ok(Some(path))
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
