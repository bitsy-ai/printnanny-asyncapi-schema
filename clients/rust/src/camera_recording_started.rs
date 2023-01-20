// CameraRecordingStarted represents a CameraRecordingStarted model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingStarted {
    #[serde(rename="recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<Box<crate::VideoRecording>>,
}

impl CameraRecordingStarted {
    pub fn new(recording: Option<crate::VideoRecording>) -> CameraRecordingStarted {
        CameraRecordingStarted {
            recording: recording.map(Box::new),
        }
    }
}
