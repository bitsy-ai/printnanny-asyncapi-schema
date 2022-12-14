// PlaybackVideo represents a PlaybackVideo model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PlaybackVideo {
    #[serde(rename="uri")]
    pub uri: String,
    #[serde(rename="src_type")]
    pub src_type: Box<crate::PlaybackSourceType>,
}

impl PlaybackVideo {
    pub fn new(uri: String, src_type: crate::PlaybackSourceType) -> PlaybackVideo {
        PlaybackVideo {
            uri,
            src_type: Box::new(src_type),
        }
    }
}
