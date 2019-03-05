extern crate serde;

use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(default = "Root::default_path")]
    pub path: String,

    #[serde(default = "Root::default_readonly")]
    pub readonly: bool,
}

impl Root {
    fn new() -> Root {
        Root {
            path: Root::default_path(),
            readonly: Root::default_readonly(),
        }
    }

    fn default_path() -> String {
        "rootpath".to_string()
    }

    fn default_readonly() -> bool {
        true
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default = "Spec::default_oci_version")]
    pub oci_version: String,

    #[serde(default = "Spec::default_hostname")]
    pub hostname: String,

    #[serde(default = "Root::new")]
    pub root: Root,
}

impl Spec {
    pub fn from_json(path: &str) -> Result<Spec, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let spec = serde_json::from_reader(reader)?;

        Ok(spec)
    }

    fn default_oci_version() -> String {
        "1.0.1-dev".to_string()
    }

    fn default_hostname() -> String {
        "cr7".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;


    #[test]
    fn from_json_creates_a_spec_from_json() {
        let contents = b"\
{
    \"ociVersion\": \"1.0.0\",
    \"hostname\": \"container-hostname\"
}";
        let mut dir = env::temp_dir();
        dir.push("from_json.json");

        let mut file = File::create(&dir).unwrap();

        let path = dir.to_str().unwrap();

        file.write_all(contents).unwrap();

        let spec = Spec::from_json(path).unwrap();
        assert_eq!(spec.oci_version, "1.0.0");
        assert_eq!(spec.hostname, "container-hostname");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn from_json_creates_a_spec_with_default_oci_version() {
        let mut dir = env::temp_dir();
        dir.push("defaults.json");

        let mut file = File::create(&dir).unwrap();
        let path = dir.to_str().unwrap();

        file.write_all(b"{}").unwrap();

        let spec = Spec::from_json(dir.to_str().unwrap()).unwrap();

        assert_eq!(spec.oci_version, Spec::default_oci_version());
        assert_eq!(spec.hostname, Spec::default_hostname());
        assert_eq!(spec.root.path, Root::default_path());
        assert_eq!(spec.root.readonly, Root::default_readonly());

        std::fs::remove_file(path).unwrap();
    }
}
