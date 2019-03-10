use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;

mod root;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    oci_version: String,
    hostname: Option<String>,
    root: root::Spec,
}

impl Spec {
    pub fn from_json(path: &str) -> Result<Spec, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let spec = serde_json::from_reader(reader)?;
        Ok(spec)
    }

    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn root_path(&self) -> &str { &self.root.path() }
    pub fn is_root_readonly(&self) -> bool { self.root.readonly() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::time::SystemTime;

    fn create_tmp_file(contents: &str) -> String {
        let sys_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let file_name = format!("config_{}.json", sys_time.subsec_millis());
        let mut file_path = env::temp_dir();
        file_path.push(file_name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        file_path.to_str().unwrap().to_string()
    }

    fn remove_tmp_file(path: &str) { std::fs::remove_file(path).unwrap(); }

    fn valid_json() -> &'static str {
        "{\"ociVersion\": \"1.0.0\", \"hostname\": \"test\", \"root\": {\"path\": \"rootpath\"}}"
    }

    #[test]
    fn test_config_file_missing() {
        let err = Spec::from_json("not_found.json");
        assert!(err.is_err(), "expected {:?} to be an error", err);
    }

    #[test]
    fn test_oci_version_field_missing() {
        let contents = "{}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_oci_version_field_invalid() {
        let contents = "{\"ociVersion\": 1}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_oci_version_field_snake_case() {
        let contents = "{\"oci_version\": \"1.0.0\"}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_hostname_field_missing() {
        let contents = "{\"ociVersion\": \"1.0.0\", \"root\": {\"path\": \"mycontainer\"}}";
        let path = create_tmp_file(contents);
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(spec.unwrap().hostname, None);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_hostname_field_invalid() {
        let contents = "{\"ociVersion\": \"1.0.0\", \"hostname\": 1, root: {\"path\": \"mycontainer\"}}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_oci_version() {
        let path = create_tmp_file(valid_json());
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(spec.unwrap().oci_version(), "1.0.0");
        remove_tmp_file(&path);
    }

    #[test]
    fn test_hostname_some() {
        let path = create_tmp_file(valid_json());
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(*spec.unwrap().hostname(), Some("test".to_string()));
        remove_tmp_file(&path);
    }

    #[test]
    fn test_hostname_none() {
        let contents = "{\"ociVersion\": \"1.0.0\", \"root\": {\"path\": \"rootpath\"}}";
        let path = create_tmp_file(&contents);
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(*spec.unwrap().hostname(), None);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_root_path() {
        let path = create_tmp_file(valid_json());
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(spec.unwrap().root_path(), "rootpath");
        remove_tmp_file(&path);
    }

    #[test]
    fn test_is_root_readonly() {
        let path = create_tmp_file(valid_json());
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(spec.unwrap().is_root_readonly(), true);
        remove_tmp_file(&path);
    }
}
