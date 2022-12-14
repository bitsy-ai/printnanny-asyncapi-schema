// WebrtcSettingsApplyReply represents a WebrtcSettingsApplyReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcSettingsApplyReply {
    #[serde(rename="request")]
    pub request: Box<crate::WebrtcSettingsApplyRequest>,
}

impl WebrtcSettingsApplyReply {
    pub fn new(request: crate::WebrtcSettingsApplyRequest) -> WebrtcSettingsApplyReply {
        WebrtcSettingsApplyReply {
            request: Box::new(request),
        }
    }
}
