// CameraRecordingLoadPartsRequest represents a CameraRecordingLoadPartsRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadPartsRequest {
    #[serde(rename="video_recording_id")]
    pub video_recording_id: String,
}

impl CameraRecordingLoadPartsRequest {
    pub fn new(video_recording_id: String) -> CameraRecordingLoadPartsRequest {
        CameraRecordingLoadPartsRequest {
            video_recording_id,
        }
    }
}
