// WebrtcSettingsApplyRequest represents a WebrtcSettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcSettingsApplyRequest {
    #[serde(rename="video_src", skip_serializing_if = "Option::is_none")]
    pub video_src: Option<Box<crate::AnonymousSchema37>>,
}

impl WebrtcSettingsApplyRequest {
    pub fn new(video_src: Option<crate::AnonymousSchema37>) -> WebrtcSettingsApplyRequest {
        WebrtcSettingsApplyRequest {
            video_src: video_src.map(Box::new),
        }
    }
}
