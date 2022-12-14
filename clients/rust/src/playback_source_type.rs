// PlaybackSourceType represents a PlaybackSourceType model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PlaybackSourceType {
    #[serde(rename="uri")]
    Uri,
    #[serde(rename="file")]
    File,
}
impl Default for PlaybackSourceType {
    fn default() -> PlaybackSourceType {
        PlaybackSourceType::Uri
    }
}