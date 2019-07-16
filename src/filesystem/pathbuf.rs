use std::path::PathBuf;

pub fn to_string(path: PathBuf) -> String {
    if let Some(str_path) = path.to_str() {
        return String::from(str_path)
    }

    return String::from("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pathbuf_to_string_returns_a_string_with_path() {
        let path = PathBuf::from("/usr/lib");
        let string_path = to_string(path);

        assert_eq!(string_path, "/usr/lib".to_string())
    }

    #[test]
    fn pathbuf_to_string_returns_an_empty_string() {
        let path = PathBuf::from("");
        let string_path = to_string(path);

        assert_eq!(string_path, "".to_string())
    }
}
