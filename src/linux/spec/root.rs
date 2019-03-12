use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
