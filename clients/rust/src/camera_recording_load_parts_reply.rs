// CameraRecordingLoadPartsReply represents a CameraRecordingLoadPartsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraRecordingLoadPartsReply {
    #[serde(rename="recording")]
    pub recording: Box<crate::VideoRecording>,
    #[serde(rename="parts")]
    pub parts: Vec<crate::VideoRecordingPart>,
}

impl CameraRecordingLoadPartsReply {
    pub fn new(recording: crate::VideoRecording, parts: Vec<crate::VideoRecordingPart>) -> CameraRecordingLoadPartsReply {
        CameraRecordingLoadPartsReply {
            recording: Box::new(recording),
            parts,
        }
    }
}
