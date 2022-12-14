// WebrtcSettingsApplyRequest represents a WebrtcSettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcSettingsApplyRequest {
    #[serde(rename="video_src")]
    pub video_src: Box<crate::VideoSource>,
}

impl WebrtcSettingsApplyRequest {
    pub fn new(video_src: crate::VideoSource) -> WebrtcSettingsApplyRequest {
        WebrtcSettingsApplyRequest {
            video_src: Box::new(video_src),
        }
    }
}
