use crate::spec::ProcessSpec;
use crate::spec::ConsoleSizeSpec;
use serde::Serialize;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Process {
    terminal: Option<bool>,
    console_size: Option<ConsoleSize>,
    cwd: PathBuf,
    env: Option<Vec<String>>,
    args: Option<Vec<String>>,
}

impl ProcessSpec for Process {
    type ConsoleSize = ConsoleSize;

    fn get_terminal(&self) -> Option<&bool> {
        self.terminal.as_ref()
    }

    fn get_terminal_clone(&self) -> Option<bool> {
        self.terminal.as_ref().cloned()
    }

    fn get_console_size(&self) -> Option<&Self::ConsoleSize> {
        self.console_size.as_ref()
    }

    fn get_console_size_clone(&self) -> Option<Self::ConsoleSize> {
        self.console_size.as_ref().cloned()
    }

    fn get_cwd(&self) -> &PathBuf {
        &self.cwd
    }

    fn get_cwd_clone(&self) -> PathBuf {
        self.cwd.clone()
    }

    fn get_env(&self) -> Option<&Vec<String>> {
        self.env.as_ref()
    }

    fn get_env_clone(&self) -> Option<Vec<String>> {
        self.env.as_ref().cloned()
    }

    fn get_args(&self) -> Option<&Vec<String>> {
        self.args.as_ref()
    }

    fn get_args_clone(&self) -> Option<Vec<String>> {
        self.args.as_ref().cloned()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleSize {
    height: u16,
    width: u16,
}

impl ConsoleSizeSpec for ConsoleSize {
    fn get_height(&self) -> u16 {
        self.height
    }

    fn get_width(&self) -> u16 {
        self.width
    }
}
