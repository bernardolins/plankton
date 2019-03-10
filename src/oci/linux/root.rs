use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    path: String,

    #[serde(default = "Spec::default_readonly")]
    readonly: bool,
}

impl Spec {
    fn default_readonly() -> bool { true }

    pub fn path(&self) -> &str { &self.path }
    pub fn readonly(&self) -> bool { self.readonly}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let spec = Spec {path: "rootpath".to_string(), readonly: false};
        assert_eq!(spec.path(), "rootpath");
    }

    #[test]
    fn test_readonly() {
        let spec = Spec {path: "rootpath".to_string(), readonly: false};
        assert_eq!(spec.readonly(), false);
    }
}
