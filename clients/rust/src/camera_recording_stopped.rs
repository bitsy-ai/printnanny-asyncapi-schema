// CameraRecordingStopped represents a CameraRecordingStopped model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingStopped {
    #[serde(rename="recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<Box<crate::VideoRecording>>,
}

impl CameraRecordingStopped {
    pub fn new(recording: Option<crate::VideoRecording>) -> CameraRecordingStopped {
        CameraRecordingStopped {
            recording: recording.map(Box::new),
        }
    }
}
