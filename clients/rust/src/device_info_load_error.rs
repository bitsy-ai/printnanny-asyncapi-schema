// DeviceInfoLoadError represents a DeviceInfoLoadError model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeviceInfoLoadError {
    #[serde(rename="error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename="subject_pattern", skip_serializing_if = "Option::is_none")]
    pub subject_pattern: Option<String>,
    #[serde(rename="request", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

impl DeviceInfoLoadError {
    pub fn new(error: Option<String>, subject_pattern: Option<String>, request: Option<String>) -> DeviceInfoLoadError {
        DeviceInfoLoadError {
            error,
            subject_pattern,
            request,
        }
    }
}
