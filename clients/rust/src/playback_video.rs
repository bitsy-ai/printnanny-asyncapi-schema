// PlaybackVideo represents a PlaybackVideo model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PlaybackVideo {
    #[serde(rename="uri")]
    pub uri: String,
    #[serde(rename="video_src", skip_serializing_if = "Option::is_none")]
    pub video_src: Option<Box<crate::VideoSourceType>>,
}

impl PlaybackVideo {
    pub fn new(uri: String, video_src: Option<crate::VideoSourceType>) -> PlaybackVideo {
        PlaybackVideo {
            uri,
            video_src: video_src.map(Box::new),
        }
    }
}
