// SystemdManagerDisableUnitsRequest represents a SystemdManagerDisableUnitsRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerDisableUnitsRequest {
    #[serde(rename="files")]
    pub files: Vec<String>,
}

impl SystemdManagerDisableUnitsRequest {
    pub fn new(files: Vec<String>) -> SystemdManagerDisableUnitsRequest {
        SystemdManagerDisableUnitsRequest {
            files,
        }
    }
}
