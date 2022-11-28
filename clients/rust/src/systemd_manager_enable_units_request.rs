// SystemdManagerEnableUnitsRequest represents a SystemdManagerEnableUnitsRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerEnableUnitsRequest {
    #[serde(rename="files")]
    files: Vec<String>,
}

impl SystemdManagerEnableUnitsRequest {
    pub fn new(files: Vec<String>) -> SystemdManagerEnableUnitsRequest {
        SystemdManagerEnableUnitsRequest {
            files,
        }
    }
}
