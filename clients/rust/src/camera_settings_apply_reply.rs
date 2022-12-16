// CameraSettingsApplyReply represents a CameraSettingsApplyReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraSettingsApplyReply {
    #[serde(rename="request")]
    pub request: Box<crate::CameraSettingsApplyRequest>,
}

impl CameraSettingsApplyReply {
    pub fn new(request: crate::CameraSettingsApplyRequest) -> CameraSettingsApplyReply {
        CameraSettingsApplyReply {
            request: Box::new(request),
        }
    }
}
