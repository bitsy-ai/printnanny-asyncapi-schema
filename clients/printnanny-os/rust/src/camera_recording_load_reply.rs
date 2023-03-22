// CameraRecordingLoadReply represents a CameraRecordingLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadReply {
    #[serde(rename="recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<Box<crate::VideoRecording>>,
    #[serde(rename="parts", skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<crate::VideoRecordingPart>>,
}

impl CameraRecordingLoadReply {
    pub fn new(recording: Option<crate::VideoRecording>, parts: Option<Vec<crate::VideoRecordingPart>>) -> CameraRecordingLoadReply {
        CameraRecordingLoadReply {
            recording: recording.map(Box::new),
            parts,
        }
    }
}
