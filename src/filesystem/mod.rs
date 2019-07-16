use std::env;

pub mod pathbuf;

const DEFAULT_PATH: &str = ".";

pub fn cwd() -> String {
    if let Some(dir) = env::current_dir().ok() {
        match dir.to_str() {
            Some(dir) => dir.to_string(),
            None => DEFAULT_PATH.to_string(),
        }
    } else {
        DEFAULT_PATH.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cwd_returns_current_dir_as_string() {
        let dir = std::env::current_dir();

        assert_eq!(dir.unwrap().to_str().unwrap().to_string(), cwd());
    }
}
