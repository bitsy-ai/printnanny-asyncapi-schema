// CameraRecordingLoadPartsReply represents a CameraRecordingLoadPartsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadPartsReply {
    #[serde(rename="recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<Box<crate::VideoRecording>>,
    #[serde(rename="parts", skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<crate::VideoRecordingPart>>,
}

impl CameraRecordingLoadPartsReply {
    pub fn new(recording: Option<crate::VideoRecording>, parts: Option<Vec<crate::VideoRecordingPart>>) -> CameraRecordingLoadPartsReply {
        CameraRecordingLoadPartsReply {
            recording: recording.map(Box::new),
            parts,
        }
    }
}
