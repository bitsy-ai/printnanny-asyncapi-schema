// CameraRecordingStarted represents a CameraRecordingStarted model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingStarted {
    #[serde(rename="recording")]
    pub recording: Box<crate::VideoRecording>,
}

impl CameraRecordingStarted {
    pub fn new(recording: crate::VideoRecording) -> CameraRecordingStarted {
        CameraRecordingStarted {
            recording: Box::new(recording),
        }
    }
}
