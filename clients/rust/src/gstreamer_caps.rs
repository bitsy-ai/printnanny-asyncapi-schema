// GstreamerCaps represents a GstreamerCaps model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GstreamerCaps {
    #[serde(rename="media_type")]
    pub media_type: String,
    #[serde(rename="format")]
    pub format: String,
    #[serde(rename="width")]
    pub width: i32,
    #[serde(rename="height")]
    pub height: i32,
}

impl GstreamerCaps {
    pub fn new(media_type: String, format: String, width: i32, height: i32) -> GstreamerCaps {
        GstreamerCaps {
            media_type,
            format,
            width,
            height,
        }
    }
}
