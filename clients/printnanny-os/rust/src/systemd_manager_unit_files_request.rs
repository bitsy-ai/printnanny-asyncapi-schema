// SystemdManagerUnitFilesRequest represents a SystemdManagerUnitFilesRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerUnitFilesRequest {
    #[serde(rename="files")]
    pub files: Vec<String>,
}

impl SystemdManagerUnitFilesRequest {
    pub fn new(files: Vec<String>) -> SystemdManagerUnitFilesRequest {
        SystemdManagerUnitFilesRequest {
            files,
        }
    }
}
