// SystemdManagerGetUnitFileStateError represents a SystemdManagerGetUnitFileStateError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerGetUnitFileStateError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerGetUnitFileStateError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerGetUnitFileStateError {
        SystemdManagerGetUnitFileStateError {
            error,
            subject_pattern,
            request,
        }
    }
}
