// HlsSettings represents a HlsSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HlsSettings {
    #[serde(rename="hls_segments")]
    pub hls_segments: String,
    #[serde(rename="hls_playlist")]
    pub hls_playlist: String,
    #[serde(rename="hls_playlist_root")]
    pub hls_playlist_root: String,
    #[serde(rename="hls_enabled", skip_serializing_if = "Option::is_none")]
    pub hls_enabled: Option<bool>,
}

impl HlsSettings {
    pub fn new(hls_segments: String, hls_playlist: String, hls_playlist_root: String, hls_enabled: Option<bool>) -> HlsSettings {
        HlsSettings {
            hls_segments,
            hls_playlist,
            hls_playlist_root,
            hls_enabled,
        }
    }
}
