mod status;
mod bundle;

use std::error::Error;
use self::status::Status;
use self::bundle::Bundle;

struct Container {
    id: String,
    status: Status,
    bundle: Bundle,
}

impl Container {
    pub fn new(id: &str, bundle_path: &str) -> Result<Container, Box<Error>> {
        let bundle = Bundle::new(bundle_path)?;

        let container = Container {
            id: id.to_string(),
            status: Status::Creating,
            bundle: bundle,
        };

        Ok(container)
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
        let container = Container::new("test_container", &create_temp_dir("oci_bundle")).unwrap();
        assert_eq!(container.id, "test_container");
        assert_eq!(container.status, Status::Creating);
    }
}
