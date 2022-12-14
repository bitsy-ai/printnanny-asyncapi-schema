// WebrtcSettingsApplyReply represents a WebrtcSettingsApplyReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebrtcSettingsApplyReply {
    #[serde(rename="request", skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<crate::WebrtcSettingsApplyRequest>>,
}

impl WebrtcSettingsApplyReply {
    pub fn new(request: Option<crate::WebrtcSettingsApplyRequest>) -> WebrtcSettingsApplyReply {
        WebrtcSettingsApplyReply {
            request: request.map(Box::new),
        }
    }
}
