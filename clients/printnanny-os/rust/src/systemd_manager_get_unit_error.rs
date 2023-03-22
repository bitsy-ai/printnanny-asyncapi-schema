// SystemdManagerGetUnitError represents a SystemdManagerGetUnitError model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: Box<crate::SystemdManagerGetUnitRequest>,
}

impl SystemdManagerGetUnitError {
    pub fn new(error: String, subject_pattern: String, request: crate::SystemdManagerGetUnitRequest) -> SystemdManagerGetUnitError {
        SystemdManagerGetUnitError {
            error,
            subject_pattern,
            request: Box::new(request),
        }
    }
}
