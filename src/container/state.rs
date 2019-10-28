use crate::Error;
use serde::Serialize;
use serde::Deserialize;
use failure::ResultExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub bundle: String,
    pub id: String,
    pub oci_version: String,
    pub pid: Option<i32>,
    pub status: String,
}

impl State {
    pub fn to_json(&self) -> Result<String, Error> {
        let json =
            serde_json::to_string_pretty(&self)
            .context("error parsing container state".to_string())?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn state_to_json_returns_the_json_version_of_state() {
        let state = State {
            bundle: "/containers/my-container-id".to_string(),
            id: "my-container-id".to_string(),
            oci_version: "1.0.1-beta1".to_string(),
            pid: Some(5327),
            status: "creating".to_string(),
        };

        let json_state = json!({
            "oci_version": "1.0.1-beta1",
            "id": "my-container-id",
            "status": "creating",
            "pid": Some(5327),
            "bundle": "/containers/my-container-id",
        });

        assert!(state.to_json().is_ok());
        assert_eq!(state.to_json().unwrap(), serde_json::to_string_pretty(&json_state).unwrap());
    }
}
