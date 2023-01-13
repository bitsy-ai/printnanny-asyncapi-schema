// WebrtcRecording represents a WebrtcRecording model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcRecording {
    #[serde(rename="media", skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<crate::WebrtcRecordingMedia>>,
    #[serde(rename="mountpoint", skip_serializing_if = "Option::is_none")]
    pub mountpoint: Option<String>,
}

impl WebrtcRecording {
    pub fn new(media: Option<Vec<crate::WebrtcRecordingMedia>>, mountpoint: Option<String>) -> WebrtcRecording {
        WebrtcRecording {
            media,
            mountpoint,
        }
    }
}
