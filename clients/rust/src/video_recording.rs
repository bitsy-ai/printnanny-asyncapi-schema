// VideoRecording represents a VideoRecording model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecording {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="recording_status", skip_serializing_if = "Option::is_none")]
    pub recording_status: Option<Box<crate::VideoRecordingStatus>>,
    #[serde(rename="recording_start", skip_serializing_if = "Option::is_none")]
    pub recording_start: Option<String>,
    #[serde(rename="recording_end", skip_serializing_if = "Option::is_none")]
    pub recording_end: Option<String>,
    #[serde(rename="mp4_file_name")]
    pub mp4_file_name: String,
    #[serde(rename="mp4_download_url", skip_serializing_if = "Option::is_none")]
    pub mp4_download_url: Option<String>,
    #[serde(rename="mp4_upload_url", skip_serializing_if = "Option::is_none")]
    pub mp4_upload_url: Option<String>,
    #[serde(rename="gcode_file_name", skip_serializing_if = "Option::is_none")]
    pub gcode_file_name: Option<String>,
    #[serde(rename="cloud_sync_status", skip_serializing_if = "Option::is_none")]
    pub cloud_sync_status: Option<Box<crate::VideoRecordingStatus>>,
    #[serde(rename="cloud_sync_start", skip_serializing_if = "Option::is_none")]
    pub cloud_sync_start: Option<String>,
    #[serde(rename="cloud_sync_end", skip_serializing_if = "Option::is_none")]
    pub cloud_sync_end: Option<String>,
}

impl VideoRecording {
    pub fn new(id: String, recording_status: Option<crate::VideoRecordingStatus>, recording_start: Option<String>, recording_end: Option<String>, mp4_file_name: String, mp4_download_url: Option<String>, mp4_upload_url: Option<String>, gcode_file_name: Option<String>, cloud_sync_status: Option<crate::VideoRecordingStatus>, cloud_sync_start: Option<String>, cloud_sync_end: Option<String>) -> VideoRecording {
        VideoRecording {
            id,
            recording_status: recording_status.map(Box::new),
            recording_start,
            recording_end,
            mp4_file_name,
            mp4_download_url,
            mp4_upload_url,
            gcode_file_name,
            cloud_sync_status: cloud_sync_status.map(Box::new),
            cloud_sync_start,
            cloud_sync_end,
        }
    }
}
