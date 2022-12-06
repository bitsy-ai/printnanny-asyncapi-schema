// SystemdManagerDisableUnitsError represents a SystemdManagerDisableUnitsError model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerDisableUnitsError {
    #[serde(rename="error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename="subject_pattern", skip_serializing_if = "Option::is_none")]
    pub subject_pattern: Option<String>,
    #[serde(rename="request", skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
}

impl SystemdManagerDisableUnitsError {
    pub fn new(error: Option<String>, subject_pattern: Option<String>, request: Option<serde_json::Value>) -> SystemdManagerDisableUnitsError {
        SystemdManagerDisableUnitsError {
            error,
            subject_pattern,
            request,
        }
    }
}
