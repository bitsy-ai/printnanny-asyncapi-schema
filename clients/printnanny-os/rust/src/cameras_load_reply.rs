// CamerasLoadReply represents a CamerasLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CamerasLoadReply {
    #[serde(rename="cameras")]
    pub cameras: Vec<crate::Camera>,
}

impl CamerasLoadReply {
    pub fn new(cameras: Vec<crate::Camera>) -> CamerasLoadReply {
        CamerasLoadReply {
            cameras,
        }
    }
}
