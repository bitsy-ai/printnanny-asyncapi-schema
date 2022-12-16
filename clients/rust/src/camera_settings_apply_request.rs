// CameraSettingsApplyRequest represents a CameraSettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraSettingsApplyRequest {
    #[serde(rename="video_src")]
    pub video_src: Box<crate::VideoSource>,
}

impl CameraSettingsApplyRequest {
    pub fn new(video_src: crate::VideoSource) -> CameraSettingsApplyRequest {
        CameraSettingsApplyRequest {
            video_src: Box::new(video_src),
        }
    }
}
