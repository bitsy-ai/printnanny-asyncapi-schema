// PrintNannyCameraSettings represents a PrintNannyCameraSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCameraSettings {
    #[serde(rename="overlay_udp_port")]
    pub overlay_udp_port: i32,
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
    #[serde(rename="video_src")]
    pub video_src: Box<crate::VideoSource>,
}

impl PrintNannyCameraSettings {
    pub fn new(overlay_udp_port: i32, preview: bool, video_framerate: i32, video_udp_port: i32, detection: crate::PrintNannyDetectionSettings, hls: crate::HlsSettings, video_src: crate::VideoSource) -> PrintNannyCameraSettings {
        PrintNannyCameraSettings {
            overlay_udp_port,
            preview,
            video_framerate,
            video_udp_port,
            detection: Box::new(detection),
            hls: Box::new(hls),
            video_src: Box::new(video_src),
        }
    }
}
