// VideoStreamSettings represents a VideoStreamSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoStreamSettings {
    #[serde(rename="camera")]
    pub camera: Box<crate::CameraSettings>,
    #[serde(rename="detection")]
    pub detection: Box<crate::DetectionSettings>,
    #[serde(rename="hls")]
    pub hls: Box<crate::HlsSettings>,
    #[serde(rename="recording")]
    pub recording: Box<crate::RecordingSettings>,
    #[serde(rename="rtp")]
    pub rtp: Box<crate::RtpSettings>,
    #[serde(rename="snapshot")]
    pub snapshot: Box<crate::SnapshotSettings>,
}

impl VideoStreamSettings {
    pub fn new(camera: crate::CameraSettings, detection: crate::DetectionSettings, hls: crate::HlsSettings, recording: crate::RecordingSettings, rtp: crate::RtpSettings, snapshot: crate::SnapshotSettings) -> VideoStreamSettings {
        VideoStreamSettings {
            camera: Box::new(camera),
            detection: Box::new(detection),
            hls: Box::new(hls),
            recording: Box::new(recording),
            rtp: Box::new(rtp),
            snapshot: Box::new(snapshot),
        }
    }
}
