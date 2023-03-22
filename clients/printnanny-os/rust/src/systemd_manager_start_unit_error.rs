// SystemdManagerStartUnitError represents a SystemdManagerStartUnitError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerStartUnitError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerStartUnitError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerStartUnitError {
        SystemdManagerStartUnitError {
            error,
            subject_pattern,
            request,
        }
    }
}
