// SystemdManagerChangeUnitRequest represents a SystemdManagerChangeUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerChangeUnitRequest {
    #[serde(rename="files")]
    files: Vec<String>,
}

impl SystemdManagerChangeUnitRequest {
    pub fn new(files: Vec<String>) -> SystemdManagerChangeUnitRequest {
        SystemdManagerChangeUnitRequest {
            files,
        }
    }
}
