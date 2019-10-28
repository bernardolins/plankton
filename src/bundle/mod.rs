mod config;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::marker::PhantomData;
use failure::ResultExt;
use crate::error::Error;
use crate::spec::Spec;
use crate::filesystem::pathbuf;

pub use self::config::Config;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub struct Bundle<S: Spec> {
    path: PathBuf,
    bundle_type: PhantomData<S>,
}

impl<S: Spec> Bundle<S> {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn open(dir: &str) -> Result<Bundle<S>, Error> {
        let bundle_path = PathBuf::from(dir);
        let path = bundle_path.canonicalize().context(format!("{:?}", bundle_path))?;
        let bundle_type = PhantomData;
        Ok(Bundle { path, bundle_type })
    }

    pub fn load_config(&self) -> Result<S, Error> {
        let config_file_path = self.path.join(CONFIG_FILE_NAME);
        let file = File::open(&config_file_path).context(format!("{:?}", config_file_path))?;
        let config_reader = BufReader::new(file);
        let spec = S::from_reader(config_reader)?;
        Ok(spec)
    }
}

pub fn load_config(bundle_dir: &str) -> Result<Config, Error> {
    let bundle_path = canonical_bundle_path(bundle_dir)?;
    let config_path = canonical_config_path(bundle_path)?;
    let config_reader = read_config_file(config_path)?;

    let config = Config::load(config_reader)?;
    Ok(config)
}

fn canonical_bundle_path(bundle_dir: &str) -> Result<PathBuf, Error> {
    let bundle_path = PathBuf::from(bundle_dir);
    let path = bundle_path.canonicalize().context(pathbuf::to_string(bundle_path))?;
    Ok(path)
}

fn canonical_config_path(bundle_path: PathBuf) -> Result<PathBuf, Error> {
    let config_file_path = bundle_path.join(CONFIG_FILE_NAME);
    let path = config_file_path.canonicalize().context(pathbuf::to_string(config_file_path))?;
    Ok(path)
}

fn read_config_file(path: PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(&path).context(pathbuf::to_string(path))?;
    Ok(BufReader::new(file))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use crate::error::Error;
    use crate::spec::Spec;
    use tempfile::tempdir;

    #[derive(Debug, PartialEq)]
    struct FakeSpec {
        oci_version: String,
        root: Option<String>,
        mounts: Vec<Option<String>>,
        process: Option<String>,
        hostname: Option<String>,
    }

    impl FakeSpec {
        fn default() -> FakeSpec {
            let none: Option<String> = None;
            let vec_none = vec![none];
            FakeSpec { oci_version: "v1.0.0".to_string(), root: None, mounts: vec_none, process: None, hostname: None }
        }
    }

    impl Spec for FakeSpec {
        type RootSpec = Option<String>;
        type MountSpec = Option<String>;
        type ProcessSpec = Option<String>;

        fn from_reader<R: Read>(_: R) -> Result<Self, Error> {
            Ok(FakeSpec::default())
        }
        fn oci_version(&self) -> &str { "v1.0.0-rc1" }
        fn hostname(&self) -> &Option<String> { &self.hostname }
        fn root(&self) -> &Self::RootSpec { &self.root }
        fn mounts(&self) -> &Vec<Self::MountSpec> { &self.mounts }
        fn process(&self) -> &Self::ProcessSpec { &self.process }
    }

    #[derive(Debug, PartialEq)]
    struct ErrorSpec {
        field: String,
        list: Vec<String>
    }

    impl Spec for ErrorSpec {
        type RootSpec = String;
        type MountSpec = String;
        type ProcessSpec = String;

        fn from_reader<R: Read>(_: R) -> Result<Self, Error> {
            Err(Error::from("invalid spec".to_string()))
        }
        fn oci_version(&self) -> &str { "v1.0.0-rc1" }
        fn hostname(&self) -> &Option<String> { &None }
        fn root(&self) -> &Self::RootSpec { &self.field }
        fn mounts(&self) -> &Vec<Self::MountSpec> { &self.list }
        fn process(&self) -> &Self::ProcessSpec { &self.field }
    }

    #[test]
    fn bundle_open_when_dir_does_not_exist() {
        let result = Bundle::<FakeSpec>::open("/some/invalid/path/to/bundle");
        assert!(result.is_err());
    }

    #[test]
    fn bundle_open_when_dir_exists() {
        let tempdir = tempdir().unwrap();
        let dir = tempdir.path().to_str().unwrap();
        let result = Bundle::<FakeSpec>::open(dir);
        assert!(result.is_ok(), "expected {:?} to be ok", &result);
    }

    #[test]
    fn bundle_load_config_when_config_not_found() {
        let tempdir = tempdir().unwrap();
        let dir = tempdir.path().to_str().unwrap();
        let bundle = Bundle::<FakeSpec>::open(dir).unwrap();
        let result = bundle.load_config();
        assert!(result.is_err(), "expected {:?} to be error", &result);
    }

    #[test]
    fn bundle_load_config_when_config_read_returns_error() {
        let tempdir = tempdir().unwrap();
        let file_path = tempdir.path().join(CONFIG_FILE_NAME);
        File::create(&file_path).unwrap();
        let dir = tempdir.path().to_str().unwrap();
        let bundle = Bundle::<ErrorSpec>::open(dir).unwrap();
        let result = bundle.load_config();
        assert!(result.is_err(), "expected {:?} to be err", &result);
        assert_eq!(format!("{:?}", result.err().unwrap()), "invalid spec")
    }

    #[test]
    fn bundle_load_config_when_config_read_returns_ok() {
        let tempdir = tempdir().unwrap();
        let file_path = tempdir.path().join(CONFIG_FILE_NAME);
        File::create(&file_path).unwrap();
        let dir = tempdir.path().to_str().unwrap();
        let bundle = Bundle::<FakeSpec>::open(dir).unwrap();
        let result = bundle.load_config();
        assert!(result.is_ok(), "expected {:?} to be err", &result);
        assert_eq!(result.ok().unwrap(), FakeSpec::default());
    }

    #[test]
    fn bundle_path_returns_the_path() {
        let tempdir = tempdir().unwrap();
        let dir = tempdir.path().to_str().unwrap();
        let bundle = Bundle::<FakeSpec>::open(dir).unwrap();
        assert_eq!(bundle.path(), PathBuf::from(dir));
    }
}
