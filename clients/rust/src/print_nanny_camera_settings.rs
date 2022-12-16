// PrintNannyCameraSettings represents a PrintNannyCameraSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCameraSettings {
    #[serde(rename="overlay_udp_port", skip_serializing_if = "Option::is_none")]
    pub overlay_udp_port: Option<i32>,
    #[serde(rename="preview", skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(rename="video_framerate", skip_serializing_if = "Option::is_none")]
    pub video_framerate: Option<i32>,
    #[serde(rename="video_height", skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i32>,
    #[serde(rename="video_width", skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i32>,
    #[serde(rename="video_udp_port", skip_serializing_if = "Option::is_none")]
    pub video_udp_port: Option<i32>,
    #[serde(rename="detection", skip_serializing_if = "Option::is_none")]
    pub detection: Option<Box<crate::PrintNannyDetectionSettings>>,
    #[serde(rename="hls", skip_serializing_if = "Option::is_none")]
    pub hls: Option<Box<crate::HlsSettings>>,
    #[serde(rename="video_src", skip_serializing_if = "Option::is_none")]
    pub video_src: Option<Box<crate::VideoSource>>,
}

impl PrintNannyCameraSettings {
    pub fn new(overlay_udp_port: Option<i32>, preview: Option<bool>, video_framerate: Option<i32>, video_height: Option<i32>, video_width: Option<i32>, video_udp_port: Option<i32>, detection: Option<crate::PrintNannyDetectionSettings>, hls: Option<crate::HlsSettings>, video_src: Option<crate::VideoSource>) -> PrintNannyCameraSettings {
        PrintNannyCameraSettings {
            overlay_udp_port,
            preview,
            video_framerate,
            video_height,
            video_width,
            video_udp_port,
            detection: detection.map(Box::new),
            hls: hls.map(Box::new),
            video_src: video_src.map(Box::new),
        }
    }
}
