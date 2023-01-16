// Camera represents a Camera model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Camera {
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "device_name")]
    pub device_name: String,
    #[serde(rename = "framerate", skip_serializing_if = "Option::is_none")]
    pub framerate: Option<i32>,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "src_type")]
    pub src_type: Box<crate::CameraSourceType>,
    #[serde(rename = "selected_caps")]
    pub selected_caps: Box<crate::GstreamerCaps>,
    #[serde(rename = "available_caps")]
    pub available_caps: Vec<crate::GstreamerCaps>,
}

impl Camera {
    pub fn new(
        index: i32,
        device_name: String,
        framerate: Option<i32>,
        label: String,
        src_type: crate::CameraSourceType,
        selected_caps: crate::GstreamerCaps,
        available_caps: Vec<crate::GstreamerCaps>,
    ) -> Camera {
        Camera {
            index,
            device_name,
            framerate,
            label,
            src_type: Box::new(src_type),
            selected_caps: Box::new(selected_caps),
            available_caps,
        }
    }
}
