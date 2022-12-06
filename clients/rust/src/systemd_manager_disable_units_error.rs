// SystemdManagerDisableUnitsError represents a SystemdManagerDisableUnitsError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerDisableUnitsError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerDisableUnitsError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerDisableUnitsError {
        SystemdManagerDisableUnitsError {
            error,
            subject_pattern,
            request,
        }
    }
}
