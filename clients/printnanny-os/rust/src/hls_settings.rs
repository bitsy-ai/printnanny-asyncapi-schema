// HlsSettings represents a HlsSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HlsSettings {
    #[serde(rename="segments")]
    pub segments: String,
    #[serde(rename="playlist")]
    pub playlist: String,
    #[serde(rename="playlist_root")]
    pub playlist_root: String,
    #[serde(rename="enabled")]
    pub enabled: bool,
}

impl HlsSettings {
    pub fn new(segments: String, playlist: String, playlist_root: String, enabled: bool) -> HlsSettings {
        HlsSettings {
            segments,
            playlist,
            playlist_root,
            enabled,
        }
    }
}
