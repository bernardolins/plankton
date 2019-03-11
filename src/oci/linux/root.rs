use std::path::Path;
use std::ffi::OsStr;
use serde::Deserialize;
use oci::error::ConfigError;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    path: String,

    #[serde(default = "Spec::default_readonly")]
    readonly: bool,
}

impl Spec {
    fn default_readonly() -> bool { true }

    pub fn path(&self) -> &str { &self.path }
    pub fn readonly(&self) -> bool { self.readonly}

    pub fn validate(&self) -> Result<(), ConfigError> {
        let path = Path::new(&self.path);
        if path.is_dir() {
            if path.file_name() == Some(OsStr::new("rootfs")) {
                Ok(())
            } else {
                Err(ConfigError::new("root path must a dir named 'rootfs'"))
            }
        } else {
            Err(ConfigError::new(&format!("{} must be an existent dir", &self.path)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;
    use std::time::SystemTime;

    fn create_tmp_dir(dir_name: &str) -> String {
        let sys_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let dir_path = format!("{}/{}", sys_time.subsec_millis(), dir_name);
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push(dir_path);
        fs::create_dir_all(&tmp_dir).unwrap();
        tmp_dir.to_str().unwrap().to_string()
    }

    fn remove_tmp_dir(tmp_path: &str) {
        let path = Path::new(tmp_path).parent().unwrap();
        if path.is_dir() {
            fs::remove_dir_all(&path.to_str().unwrap()).unwrap();
        }
    }

    #[test]
    fn test_path() {
        let spec = Spec {path: "rootfs".to_string(), readonly: false};
        assert_eq!(spec.path(), "rootfs");
    }

    #[test]
    fn test_readonly() {
        let spec = Spec {path: "rootfs".to_string(), readonly: false};
        assert_eq!(spec.readonly(), false);
    }

    #[test]
    fn test_validate() {
        let table = vec![
            (create_tmp_dir("rootfs"), true),
            (create_tmp_dir("not_rootfs"), false),
            ("/unexistent/path/rootfs".to_string(), false),
        ];

        for (path, expected) in table {
            let spec = Spec{path: path.clone(), readonly: false};
            assert_eq!(spec.validate().is_ok(), expected, "expected {:?} to be ok", spec);
            remove_tmp_dir(&path);
        }

    }
}
