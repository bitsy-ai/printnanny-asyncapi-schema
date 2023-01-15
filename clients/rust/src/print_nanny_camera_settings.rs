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
    #[serde(rename="snapshot_enabled")]
    pub snapshot_enabled: bool,
    #[serde(rename="snapshot_location")]
    pub snapshot_location: String,
    #[serde(rename="hls_segments")]
    pub hls_segments: String,
    #[serde(rename="hls_playlist")]
    pub hls_playlist: String,
    #[serde(rename="hls_playlist_root")]
    pub hls_playlist_root: String,
    #[serde(rename="hls_enabled")]
    pub hls_enabled: bool,
    #[serde(rename="camera")]
    pub camera: Box<crate::Camera>,
}

impl PrintNannyCameraSettings {
    pub fn new(overlay_udp_port: i32, record_video: bool, cloud_backup: bool, preview: bool, video_framerate: i32, video_udp_port: i32, detection: crate::PrintNannyDetectionSettings, snapshot_enabled: bool, snapshot_location: String, hls_segments: String, hls_playlist: String, hls_playlist_root: String, hls_enabled: bool, camera: crate::Camera) -> PrintNannyCameraSettings {
        PrintNannyCameraSettings {
            overlay_udp_port,
            record_video,
            cloud_backup,
            preview,
            video_framerate,
            video_udp_port,
            detection: Box::new(detection),
            snapshot_enabled,
            snapshot_location,
            hls_segments,
            hls_playlist,
            hls_playlist_root,
            hls_enabled,
            camera: Box::new(camera),
        }
    }
}
