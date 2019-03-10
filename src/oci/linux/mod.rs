use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;

pub mod spec;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub oci_version: String,

    pub hostname: Option<String>,

    #[serde(default = "spec::Root::default")]
    pub root: spec::Root,

    #[serde(default = "spec::Process::default")]
    pub process: spec::Process,
}

impl Spec {
    pub fn from_json(path: &str) -> Result<Spec, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let spec = serde_json::from_reader(reader)?;
        Ok(spec)
    }
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

    fn remove_tmp_file(path: &str) {
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_config_file_missing() {
        let err = Spec::from_json("not_found.json");
        assert!(err.is_err(), "expected {:?} to be an error", err);
    }

    #[test]
    fn test_oci_version_missing() {
        let contents = "{}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_oci_version_invalid() {
        let contents = "{\"ociVersion\": 1}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_oci_version_snake_case() {
        let contents = "{\"oci_version\": \"1.0.0\"}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_hostname_missing() {
        let contents = "{\"ociVersion\": \"1.0.0\"}";
        let path = create_tmp_file(contents);
        let spec = Spec::from_json(&path);
        assert!(spec.is_ok(), "expected {:?} to be ok", spec);
        assert_eq!(spec.unwrap().hostname, None);
        remove_tmp_file(&path);
    }

    #[test]
    fn test_hostname_invalid() {
        let contents = "{\"ociVersion\": \"1.0.0\", \"hostname\": 1}";
        let path = create_tmp_file(contents);
        let err = Spec::from_json(&path);
        assert!(err.is_err(), "expected {:?} to be an error", err);
        remove_tmp_file(&path);
    }
}
