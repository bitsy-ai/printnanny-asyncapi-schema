// CameraStatus represents a CameraStatus model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraStatus {
    #[serde(rename="streaming")]
    pub streaming: bool,
    #[serde(rename="recording")]
    pub recording: bool,
}

impl CameraStatus {
    pub fn new(streaming: bool, recording: bool) -> CameraStatus {
        CameraStatus {
            streaming,
            recording,
        }
    }
}
