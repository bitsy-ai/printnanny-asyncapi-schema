// CameraRecordingLoadReply represents a CameraRecordingLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadReply {
    #[serde(rename="recordings")]
    pub recordings: Vec<crate::VideoRecording>,
    #[serde(rename="current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<crate::VideoRecording>>,
}

impl CameraRecordingLoadReply {
    pub fn new(recordings: Vec<crate::VideoRecording>, current: Option<crate::VideoRecording>) -> CameraRecordingLoadReply {
        CameraRecordingLoadReply {
            recordings,
            current: current.map(Box::new),
        }
    }
}
