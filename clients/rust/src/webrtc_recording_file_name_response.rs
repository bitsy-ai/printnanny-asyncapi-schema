// WebrtcRecordingFileNameResponse represents a WebrtcRecordingFileNameResponse model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcRecordingFileNameResponse {
    #[serde(rename="file_name")]
    pub file_name: String,
    #[serde(rename="ts")]
    pub ts: String,
}

impl WebrtcRecordingFileNameResponse {
    pub fn new(file_name: String, ts: String) -> WebrtcRecordingFileNameResponse {
        WebrtcRecordingFileNameResponse {
            file_name,
            ts,
        }
    }
}
