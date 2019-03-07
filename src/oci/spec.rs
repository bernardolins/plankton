use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(default = "Process::default_terminal")]
    pub terminal: bool,

    #[serde(default = "Process::default_args")]
    pub args: Vec<String>,

    #[serde(default = "Process::default_env")]
    pub env: Vec<String>,
}

impl Process {
    pub fn default() -> Process {
        Process {
            terminal: Process::default_terminal(),
            args: Process::default_args(),
            env: Process::default_env(),
        }
    }

    fn default_terminal() -> bool { true }
    fn default_args() -> Vec<String> { vec!("sh".to_string()) }
    fn default_env() -> Vec<String> { Vec::new() }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(default = "Root::default_path")]
    pub path: String,

    #[serde(default = "Root::default_readonly")]
    pub readonly: bool,
}

impl Root {
    pub fn default() -> Root {
        Root {
            path: Root::default_path(),
            readonly: Root::default_readonly(),
        }
    }

    fn default_path() -> String { "rootpath".to_string() }
    fn default_readonly() -> bool { true }
}
