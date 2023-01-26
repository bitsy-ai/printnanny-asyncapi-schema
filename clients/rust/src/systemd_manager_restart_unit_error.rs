// SystemdManagerRestartUnitError represents a SystemdManagerRestartUnitError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerRestartUnitError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerRestartUnitError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerRestartUnitError {
        SystemdManagerRestartUnitError {
            error,
            subject_pattern,
            request,
        }
    }
}
