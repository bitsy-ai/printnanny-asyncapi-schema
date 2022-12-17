// VideoSource represents a union of types: Camera, PlaybackVideo
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum VideoSource {
    #[serde(rename="Camera")]
    Camera(crate::Camera),
    #[serde(rename="PlaybackVideo")]
    PlaybackVideo(crate::PlaybackVideo),
}

