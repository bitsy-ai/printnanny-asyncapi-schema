// PrintNannyCameraSettings represents a PrintNannyCameraSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCameraSettings {
    #[serde(rename="overlay_udp_port")]
    pub overlay_udp_port: i32,
    #[serde(rename="record_video")]
    pub record_video: bool,
    #[serde(rename="cloud_backup")]
    pub cloud_backup: bool,
    #[serde(rename="preview")]
    pub preview: bool,
    #[serde(rename="video_framerate")]
    pub video_framerate: i32,
    #[serde(rename="video_udp_port")]
    pub video_udp_port: i32,
    #[serde(rename="detection")]
    pub detection: Box<crate::PrintNannyDetectionSettings>,
    #[serde(rename="hls")]
    pub hls: Box<crate::HlsSettings>,
    #[serde(rename="camera", skip_serializing_if = "Option::is_none")]
    pub camera: Option<Box<crate::Camera>>,
}

impl PrintNannyCameraSettings {
    pub fn new(overlay_udp_port: i32, record_video: bool, cloud_backup: bool, preview: bool, video_framerate: i32, video_udp_port: i32, detection: crate::PrintNannyDetectionSettings, hls: crate::HlsSettings, camera: Option<crate::Camera>) -> PrintNannyCameraSettings {
        PrintNannyCameraSettings {
            overlay_udp_port,
            record_video,
            cloud_backup,
            preview,
            video_framerate,
            video_udp_port,
            detection: Box::new(detection),
            hls: Box::new(hls),
            camera: camera.map(Box::new),
        }
    }
}
