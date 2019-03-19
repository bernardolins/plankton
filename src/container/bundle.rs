use std::path::PathBuf;
use std::error::Error;

#[derive(Debug)]
pub struct Bundle {
    path: PathBuf,
    config_file: PathBuf,
    rootfs: PathBuf,
}

impl Bundle {
    pub fn new(path: &str) -> Result<Bundle, Box<Error>> {
        let path = PathBuf::from(path).canonicalize()?;
        let config_file_path = path.join("config.json");
        let rootfs_path = path.join("rootfs");

        let bundle = Bundle {
            path: path,
            config_file: config_file_path,
            rootfs: rootfs_path,
        };

        Ok(bundle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    fn create_temp_dir(dir_name: &str) -> String {
        let tmp_dir = env::temp_dir().join(dir_name);
        fs::create_dir_all(&tmp_dir).unwrap();
        tmp_dir.to_str().unwrap().to_string()
    }

    #[test]
    fn test_new() {
        let table = vec![
            (Bundle::new(&create_temp_dir("bundle")), true),
            (Bundle::new("/some/invalid/dir"), false),

        ];

        for (bundle, expected) in table {
            assert_eq!(bundle.is_ok(), expected, "expect {:?} to be ok", bundle);
        }
    }
}
