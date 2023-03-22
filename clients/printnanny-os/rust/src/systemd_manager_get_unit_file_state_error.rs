// SystemdManagerGetUnitFileStateError represents a SystemdManagerGetUnitFileStateError model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitFileStateError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
    #[serde(rename="request")]
    pub request: Box<crate::SystemdManagerGetUnitRequest>,
}

impl SystemdManagerGetUnitFileStateError {
    pub fn new(error: String, subject_pattern: String, request: crate::SystemdManagerGetUnitRequest) -> SystemdManagerGetUnitFileStateError {
        SystemdManagerGetUnitFileStateError {
            error,
            subject_pattern,
            request: Box::new(request),
        }
    }
}
