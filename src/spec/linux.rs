use std::io::BufRead;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    oci_version: String,
    hostname: Option<String>,
    root: Root,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Root {
    path: String,

    #[serde(default = "Root::default_readonly")]
    readonly: bool,
}

impl Spec {
    pub fn new<R: BufRead>(reader: R) -> Result<Spec, Box<Error>> {
        let spec: Spec = serde_json::from_reader(reader)?;
        Ok(spec)
    }

    pub fn oci_version(&self) -> &str { &self.oci_version }
    pub fn hostname(&self) -> &Option<String> { &self.hostname }
    pub fn root_path(&self) -> &str { &self.root.path }
    pub fn is_root_readonly(&self) -> bool { self.root.readonly }
}

impl Root {
    fn default_readonly() -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        fn valid_base(root: &str) -> String { json_template("\"1.0.0\"", "\"myhost\"", root) }
        fn valid_root() -> String { root("true", "\"rootfs\"") }

        fn json_template(oci_version: &str, hostname: &str, root: &str) -> String {
            format!(
                r#"{{"ociVersion": {}, "hostname": {}, "root": {}}}"#,
                oci_version, hostname, root
            )
        }

        fn root(readonly: &str, path: &str) -> String {
            format!(
                r#"{{"readonly": {}, "path": {}}}"#,
                readonly, path
            )
        }

        #[test]
        fn test_base() {
            let table = vec![
                (json_template("\"1.0.0\"", "\"myhost\"", &valid_root()), true),
                (json_template("0", "\"myhost\"", &valid_root()), false),
                (json_template("\"1.0.0\"", "0", &valid_root()), false),
            ];

            for (input, expect) in table {
                assert_eq!(Spec::new(input.as_bytes()).is_ok(), expect, "expect {} to be ok", input);
            }
        }

        #[test]
        fn test_root() {
            let table = vec![
                (valid_base(&root("true", "\"rootfs\"")), true),
                (valid_base(&root("\"true\"", "\"rootfs\"")), false),
                (valid_base(&root("\"true\"", "1234567")), false),
            ];

            for (input, expect) in table {
                assert_eq!(Spec::new(input.as_bytes()).is_ok(), expect, "expect {} to be ok", input);
            }
        }
    }

    fn example_spec() -> Spec {
        Spec {
            oci_version: "1.0.0".to_string(),
            hostname: Some("myhost".to_string()),
            root: Root {
                path: "rootfs".to_string(),
                readonly: true
            }
        }
    }

    #[test]
    fn test_oci_version() { assert!(example_spec().oci_version() == "1.0.0".to_string()) }

    #[test]
    fn test_hostname() { assert!(example_spec().hostname() == &Some("myhost".to_string())) }

    #[test]
    fn test_root_path() { assert!(example_spec().root_path() == "rootfs".to_string()) }

    #[test]
    fn test_is_root_readonly() { assert!(example_spec().is_root_readonly() == true) }
}
