use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::container::Container;
use crate::error::Error;

const ENVIRONMENT_ROOT: &str = "/run/cr7";

pub fn save(container: &Container) -> Result<(), Error> {
    let path = environment_path(&container.id);
    let mut file = File::create(path)?;
    let json = container.to_json()?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn load(container_id: &str) -> Result<Container, Error> {
    let path = environment_path(&container_id);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let container = Container::from_reader(reader) ?;

    Ok(container)
}

fn environment_path(container_id: &str) -> PathBuf {
    let environment_root_path = Path::new(ENVIRONMENT_ROOT);
    environment_root_path.join(container_id)
}
