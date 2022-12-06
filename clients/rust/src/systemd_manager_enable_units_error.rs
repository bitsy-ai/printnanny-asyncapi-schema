// SystemdManagerEnableUnitsError represents a SystemdManagerEnableUnitsError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerEnableUnitsError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: serde_json::Value,
}

impl SystemdManagerEnableUnitsError {
    pub fn new(error: String, subject_pattern: String, request: serde_json::Value) -> SystemdManagerEnableUnitsError {
        SystemdManagerEnableUnitsError {
            error,
            subject_pattern,
            request,
        }
    }
}
