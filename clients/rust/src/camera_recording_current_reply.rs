// CameraRecordingCurrentReply represents a CameraRecordingCurrentReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraRecordingCurrentReply {
    #[serde(rename="current_recording", skip_serializing_if = "Option::is_none")]
    pub current_recording: Option<Box<crate::VideoRecording>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl CameraRecordingCurrentReply {
    pub fn new(current_recording: Option<crate::VideoRecording>, additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> CameraRecordingCurrentReply {
        CameraRecordingCurrentReply {
            current_recording: current_recording.map(Box::new),
            additional_properties,
        }
    }
}
