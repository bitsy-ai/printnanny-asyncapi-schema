// CameraRecordingLoadRequest represents a CameraRecordingLoadRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadRequest {
    #[serde(rename="video_recording_id", skip_serializing_if = "Option::is_none")]
    pub video_recording_id: Option<String>,
}

impl CameraRecordingLoadRequest {
    pub fn new(video_recording_id: Option<String>) -> CameraRecordingLoadRequest {
        CameraRecordingLoadRequest {
            video_recording_id,
        }
    }
}
