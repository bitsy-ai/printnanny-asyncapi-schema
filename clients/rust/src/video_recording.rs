// VideoRecording represents a VideoRecording model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecording {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="recording_file_name")]
    pub recording_file_name: String,
    #[serde(rename="gcode_file-name", skip_serializing_if = "Option::is_none")]
    pub gcode_file_minus_name: Option<String>,
    #[serde(rename="ts")]
    pub ts: i64,
    #[serde(rename="cloud_sync_done")]
    pub cloud_sync_done: bool,
}

impl VideoRecording {
    pub fn new(id: String, recording_file_name: String, gcode_file_minus_name: Option<String>, ts: i64, cloud_sync_done: bool) -> VideoRecording {
        VideoRecording {
            id,
            recording_file_name,
            gcode_file_minus_name,
            ts,
            cloud_sync_done,
        }
    }
}
