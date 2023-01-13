// WebrtcRecordingMedia represents a WebrtcRecordingMedia model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcRecordingMedia {
    #[serde(rename="mid")]
    pub mid: String,
    #[serde(rename="file_name")]
    pub file_name: String,
}

impl WebrtcRecordingMedia {
    pub fn new(mid: String, file_name: String) -> WebrtcRecordingMedia {
        WebrtcRecordingMedia {
            mid,
            file_name,
        }
    }
}
