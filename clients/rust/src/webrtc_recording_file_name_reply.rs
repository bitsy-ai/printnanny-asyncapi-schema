// WebrtcRecordingFileNameReply represents a WebrtcRecordingFileNameReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcRecordingFileNameReply {
    #[serde(rename="file_name")]
    pub file_name: String,
    #[serde(rename="ts")]
    pub ts: String,
}

impl WebrtcRecordingFileNameReply {
    pub fn new(file_name: String, ts: String) -> WebrtcRecordingFileNameReply {
        WebrtcRecordingFileNameReply {
            file_name,
            ts,
        }
    }
}
