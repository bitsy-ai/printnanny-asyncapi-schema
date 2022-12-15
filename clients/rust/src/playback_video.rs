// PlaybackVideo represents a PlaybackVideo model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PlaybackVideo {
    #[serde(rename="cover")]
    pub cover: String,
    #[serde(rename="display_name")]
    pub display_name: String,
    #[serde(rename="uri")]
    pub uri: String,
    #[serde(rename="src_type")]
    pub src_type: Box<crate::PlaybackSourceType>,
}

impl PlaybackVideo {
    pub fn new(cover: String, display_name: String, uri: String, src_type: crate::PlaybackSourceType) -> PlaybackVideo {
        PlaybackVideo {
            cover,
            display_name,
            uri,
            src_type: Box::new(src_type),
        }
    }
}
