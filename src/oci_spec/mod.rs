use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;

mod root;
mod process;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OCISpec {
    #[serde(default = "OCISpec::default_oci_version")]
    pub oci_version: String,

    #[serde(default = "OCISpec::default_hostname")]
    pub hostname: String,

    #[serde(default = "root::Root::new")]
    pub root: root::Root,

    #[serde(default = "process::Process::new")]
    pub process: process::Process,
}

impl OCISpec {
    pub fn from_json(path: &str) -> Result<OCISpec, Box<Error>> {
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

        let spec = OCISpec::from_json(path).unwrap();
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

        let spec = OCISpec::from_json(dir.to_str().unwrap()).unwrap();

        assert_eq!(spec.oci_version, OCISpec::default_oci_version());
        assert_eq!(spec.hostname, OCISpec::default_hostname());
        assert_eq!(spec.root.path, root::Root::default_path());
        assert_eq!(spec.root.readonly, root::Root::default_readonly());

        assert_eq!(spec.process.terminal, process::Process::default_terminal());
        assert_eq!(spec.process.args, process::Process::default_args());
        assert_eq!(spec.process.env, process::Process::default_env());

        std::fs::remove_file(path).unwrap();
    }
}
