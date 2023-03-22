// SystemdManagerStopUnitError represents a SystemdManagerStopUnitError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerStopUnitError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerStopUnitError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerStopUnitError {
        SystemdManagerStopUnitError {
            error,
            subject_pattern,
            request,
        }
    }
}
