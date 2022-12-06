// SystemdManagerGetUnitError represents a SystemdManagerGetUnitError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerGetUnitError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerGetUnitError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerGetUnitError {
        SystemdManagerGetUnitError {
            error,
            subject_pattern,
            request,
        }
    }
}
