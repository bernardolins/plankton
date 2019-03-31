use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::container::Container;
use crate::error::Error;

const ENVIRONMENT_ROOT: &str = "/run/cr7";

pub fn save(container: &Container) -> Result<(), Error> {
    fs::create_dir_all(ENVIRONMENT_ROOT)?;
    let path = environment_path(&container.id);
    let mut file = File::create(path)?;
    let json = container.to_json()?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn load(container_id: &str) -> Result<Option<Container>, Error> {
    fs::create_dir_all(ENVIRONMENT_ROOT)?;
    let path = environment_path(&container_id);

    let file = match File::open(&path) {
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(None)
            } else {
                return Err(Error::from(err))
            }
        },
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let container = Container::from_reader(reader) ?;

    Ok(Some(container))
}

fn environment_path(container_id: &str) -> PathBuf {
    let environment_root_path = Path::new(ENVIRONMENT_ROOT);
    environment_root_path.join(container_id)
}
