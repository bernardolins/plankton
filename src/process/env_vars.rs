use crate::Error;
use crate::spec::PosixSpec;
use crate::spec::FromSpec;
use failure::ResultExt;

pub struct EnvVars {
    env_vars: Vec<(String, String)>,
}

impl EnvVars {
    pub fn empty() -> EnvVars {
        EnvVars {
            env_vars: Vec::new(),
        }
    }

    fn add_var(&mut self, var: (String, String)) {
        self.env_vars.push(var);
    }
}

impl FromSpec<PosixSpec> for EnvVars {
    fn from_spec(spec: PosixSpec) -> Result<EnvVars, Error> {
        let mut vars = EnvVars::empty();
        let error_message = "environment variable must have 'KEY=VALUE' format";
        if let Some(env_vars) = spec.process().env() {
            for env_var in env_vars {
                let mut splitted_env: Vec<&str> = env_var.split("=").collect();
                if splitted_env.len() != 2 {
                    Err(Error::from(error_message.to_string())).context(env_var.to_string())?
                }
                let k = String::from(splitted_env.remove(0));
                let v = String::from(splitted_env.remove(0));
                if k.is_empty() {
                    Err(Error::from(error_message.to_string())).context(env_var.to_string())?
                }
                vars.add_var((k, v))
            }
        }
        Ok(vars)
    }
}
