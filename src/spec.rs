use std::path::PathBuf;

pub trait Spec {
    type Mount: MountSpec;
    type Root: RootSpec;
    type Process: ProcessSpec;

    #[cfg(target_os = "linux")]
    type Linux: LinuxSpec;

    fn get_root(&self) -> Option<&Self::Root>;
    fn get_root_clone(&self) -> Option<Self::Root>;
    fn get_mounts(&self) -> Option<&Vec<Self::Mount>>;
    fn get_mounts_clone(&self) -> Option<Vec<Self::Mount>>;
    fn get_process(&self) -> Option<&Self::Process>;
    fn get_process_clone(&self) -> Option<Self::Process>;

    #[cfg(target_os = "linux")]
    fn get_linux(&self) -> Option<&Self::Linux>;
    fn get_linux_clone(&self) -> Option<Self::Linux>;
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

pub trait LinuxSpec {
    type Namespace: NamespaceSpec;

    fn get_namespaces(&self) -> Option<&Vec<Self::Namespace>>;
    fn get_namespaces_clone(&self) -> Option<Vec<Self::Namespace>>;
}

pub trait NamespaceSpec {
    fn get_type(&self) -> &String;
    fn get_type_clone(&self) -> String;

    fn get_path(&self) -> Option<&PathBuf>;
    fn get_path_clone(&self) -> Option<PathBuf>;
}
