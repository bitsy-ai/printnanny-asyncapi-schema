// DeviceInfoLoadError represents a DeviceInfoLoadError model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeviceInfoLoadError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
}

impl DeviceInfoLoadError {
    pub fn new(error: String, subject_pattern: String) -> DeviceInfoLoadError {
        DeviceInfoLoadError {
            error,
            subject_pattern,
        }
    }
}
