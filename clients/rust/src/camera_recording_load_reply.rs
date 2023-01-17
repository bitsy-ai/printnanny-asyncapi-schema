// CameraRecordingLoadReply represents a CameraRecordingLoadReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraRecordingLoadReply {
    #[serde(rename="recordings")]
    pub recordings: Vec<crate::VideoRecording>,
    #[serde(rename="current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<crate::VideoRecording>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl CameraRecordingLoadReply {
    pub fn new(recordings: Vec<crate::VideoRecording>, current: Option<crate::VideoRecording>, additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> CameraRecordingLoadReply {
        CameraRecordingLoadReply {
            recordings,
            current: current.map(Box::new),
            additional_properties,
        }
    }
}
