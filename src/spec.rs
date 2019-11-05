use std::path::PathBuf;

pub trait Spec {
    type Mount: MountSpec;
    type Root: RootSpec;
    type Process: ProcessSpec;

    fn get_root(&self) -> Option<&Self::Root>;
    fn get_root_clone(&self) -> Option<Self::Root>;
    fn get_mounts(&self) -> Option<&Vec<Self::Mount>>;
    fn get_mounts_clone(&self) -> Option<Vec<Self::Mount>>;
    fn get_process(&self) -> Option<&Self::Process>;
    fn get_process_clone(&self) -> Option<Self::Process>;
}

pub trait RootSpec {
    fn get_path(&self) -> &PathBuf;
    fn get_path_clone(&self) -> PathBuf;
    fn get_readonly(&self) -> Option<&bool>;
    fn get_readonly_clone(&self) -> Option<bool>;
}

pub trait MountSpec {
    fn get_destination(&self) -> &PathBuf;
    fn get_destination_clone(&self) -> PathBuf;
    fn get_source(&self) -> Option<&String>;
    fn get_source_clone(&self) -> Option<String>;
    fn get_options(&self) -> Option<&Vec<String>>;
    fn get_options_clone(&self) -> Option<Vec<String>>;
    #[cfg(target_os = "linux")]
    fn get_type(&self) -> Option<&String>;
    #[cfg(target_os = "linux")]
    fn get_type_clone(&self) -> Option<String>;
}

pub trait ProcessSpec {
    type ConsoleSize: ConsoleSizeSpec;

    fn get_terminal(&self) -> Option<&bool>;
    fn get_terminal_clone(&self) -> Option<bool>;
    fn get_console_size(&self) -> Option<&Self::ConsoleSize>;
    fn get_console_size_clone(&self) -> Option<Self::ConsoleSize>;
    fn get_cwd(&self) -> &PathBuf;
    fn get_cwd_clone(&self) -> PathBuf;
    fn get_env(&self) -> Option<&Vec<String>>;
    fn get_env_clone(&self) -> Option<Vec<String>>;
    fn get_args(&self) -> Option<&Vec<String>>;
    fn get_args_clone(&self) -> Option<Vec<String>>;
}

pub trait ConsoleSizeSpec {
    fn get_height(&self) -> u32;
    fn get_width(&self) -> u32;
}
