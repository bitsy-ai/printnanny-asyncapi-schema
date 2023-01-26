// SystemdManagerEnableUnitsError represents a SystemdManagerEnableUnitsError model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerEnableUnitsError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: Box<crate::SystemdManagerUnitFilesRequest>,
}

impl SystemdManagerEnableUnitsError {
    pub fn new(error: String, subject_pattern: String, request: crate::SystemdManagerUnitFilesRequest) -> SystemdManagerEnableUnitsError {
        SystemdManagerEnableUnitsError {
            error,
            subject_pattern,
            request: Box::new(request),
        }
    }
}
