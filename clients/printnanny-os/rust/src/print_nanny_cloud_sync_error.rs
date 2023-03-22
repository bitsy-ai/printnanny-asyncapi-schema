// PrintNannyCloudSyncError represents a PrintNannyCloudSyncError model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudSyncError {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="subject_pattern")]
    pub subject_pattern: String,
}

impl PrintNannyCloudSyncError {
    pub fn new(error: String, subject_pattern: String) -> PrintNannyCloudSyncError {
        PrintNannyCloudSyncError {
            error,
            subject_pattern,
        }
    }
}
