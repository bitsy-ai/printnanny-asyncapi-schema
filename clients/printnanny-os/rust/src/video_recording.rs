// VideoRecording represents a VideoRecording model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecording {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="cloud_sync_done")]
    pub cloud_sync_done: bool,
    #[serde(rename="dir")]
    pub dir: String,
    #[serde(rename="gcode_file_name", skip_serializing_if = "Option::is_none")]
    pub gcode_file_name: Option<String>,
    #[serde(rename="recording_start", skip_serializing_if = "Option::is_none")]
    pub recording_start: Option<String>,
    #[serde(rename="recording_end", skip_serializing_if = "Option::is_none")]
    pub recording_end: Option<String>,
}

impl VideoRecording {
    pub fn new(id: String, cloud_sync_done: bool, dir: String, gcode_file_name: Option<String>, recording_start: Option<String>, recording_end: Option<String>) -> VideoRecording {
        VideoRecording {
            id,
            cloud_sync_done,
            dir,
            gcode_file_name,
            recording_start,
            recording_end,
        }
    }
}
