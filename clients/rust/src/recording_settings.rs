// RecordingSettings represents a RecordingSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RecordingSettings {
    #[serde(rename="path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename="auto_start")]
    pub auto_start: bool,
    #[serde(rename="cloud_sync")]
    pub cloud_sync: bool,
}

impl RecordingSettings {
    pub fn new(path: Option<String>, auto_start: bool, cloud_sync: bool) -> RecordingSettings {
        RecordingSettings {
            path,
            auto_start,
            cloud_sync,
        }
    }
}
