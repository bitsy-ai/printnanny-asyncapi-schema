// VideoRecording represents a VideoRecording model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecording {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="recording_status")]
    pub recording_status: Box<crate::VideoRecordingStatus>,
    #[serde(rename="recording_start", skip_serializing_if = "Option::is_none")]
    pub recording_start: Option<String>,
    #[serde(rename="recording_end", skip_serializing_if = "Option::is_none")]
    pub recording_end: Option<String>,
    #[serde(rename="mp4_file_name")]
    pub mp4_file_name: String,
    #[serde(rename="gcode_file-name", skip_serializing_if = "Option::is_none")]
    pub gcode_file_minus_name: Option<String>,
    #[serde(rename="cloud_sync_status")]
    pub cloud_sync_status: Box<crate::VideoRecordingStatus>,
    #[serde(rename="cloud_sync_start", skip_serializing_if = "Option::is_none")]
    pub cloud_sync_start: Option<String>,
    #[serde(rename="cloud_sync_end", skip_serializing_if = "Option::is_none")]
    pub cloud_sync_end: Option<String>,
}

impl VideoRecording {
    pub fn new(id: String, recording_status: crate::VideoRecordingStatus, recording_start: Option<String>, recording_end: Option<String>, mp4_file_name: String, gcode_file_minus_name: Option<String>, cloud_sync_status: crate::VideoRecordingStatus, cloud_sync_start: Option<String>, cloud_sync_end: Option<String>) -> VideoRecording {
        VideoRecording {
            id,
            recording_status: Box::new(recording_status),
            recording_start,
            recording_end,
            mp4_file_name,
            gcode_file_minus_name,
            cloud_sync_status: Box::new(cloud_sync_status),
            cloud_sync_start,
            cloud_sync_end,
        }
    }
}
