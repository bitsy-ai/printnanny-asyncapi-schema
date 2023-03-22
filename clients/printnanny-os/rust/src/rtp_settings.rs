// RtpSettings represents a RtpSettings model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RtpSettings {
    #[serde(rename="video_udp_port")]
    pub video_udp_port: i32,
    #[serde(rename="overlay_udp_port")]
    pub overlay_udp_port: i32,
}

impl RtpSettings {
    pub fn new(video_udp_port: i32, overlay_udp_port: i32) -> RtpSettings {
        RtpSettings {
            video_udp_port,
            overlay_udp_port,
        }
    }
}
