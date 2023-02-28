// CameraRecordingLoadPartsRequest represents a CameraRecordingLoadPartsRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadPartsRequest {
    #[serde(rename="video_recording_id", skip_serializing_if = "Option::is_none")]
    pub video_recording_id: Option<String>,
}

impl CameraRecordingLoadPartsRequest {
    pub fn new(video_recording_id: Option<String>) -> CameraRecordingLoadPartsRequest {
        CameraRecordingLoadPartsRequest {
            video_recording_id,
        }
    }
}
