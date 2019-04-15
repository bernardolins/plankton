#[derive(Debug)]
pub struct Error {
    src_path: String,
    dst_path: String,
    fs_type: String,
    message: String,
}

impl Error {
    pub fn new(src_path: Option<&str>, dst_path: &str, filesystem: Option<&str>, message: &str) -> Error {
        let src = src_path.unwrap_or("");
        let dst = dst_path.to_string();
        let fs = filesystem.unwrap_or("");
        let msg = message.to_string();

        Error {
            src_path: src.to_string(),
            dst_path: dst,
            fs_type: fs.to_string(),
            message: msg,
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "src: {}, dst: {}, filesystem: {} -- {}",
            self.src_path,
            self.dst_path,
            self.fs_type,
            self.message
        )
    }
}
