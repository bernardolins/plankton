use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
    pub fn new() -> Process {
        Process {
            terminal: Process::default_terminal(),
            args: Process::default_args(),
            env: Process::default_env(),
        }
    }

    pub fn default_terminal() -> bool {
        true
    }

    pub fn default_args() -> Vec<String> {
        vec!("sh".to_string())
    }

    pub fn default_env() -> Vec<String> {
        Vec::new()
    }
}
