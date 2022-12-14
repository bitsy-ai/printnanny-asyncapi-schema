// VideoSource represents a union of types: Camera0, PlaybackVideo1
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VideoSource {
    #[serde(rename="Camera0")]
    Camera0(crate::Camera),
    #[serde(rename="PlaybackVideo1")]
    PlaybackVideo1(crate::PlaybackVideo),
}

