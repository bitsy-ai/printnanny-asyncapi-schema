// Camera represents a Camera model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Camera {
    #[serde(rename="availableCaps")]
    pub available_caps: Vec<crate::GstreamerCaps>,
    #[serde(rename="index")]
    pub index: i32,
    #[serde(rename="device_name")]
    pub device_name: String,
    #[serde(rename="label")]
    pub label: String,
    #[serde(rename="src_type")]
    pub src_type: Box<crate::CameraSourceType>,
}

impl Camera {
    pub fn new(available_caps: Vec<crate::GstreamerCaps>, index: i32, device_name: String, label: String, src_type: crate::CameraSourceType) -> Camera {
        Camera {
            available_caps,
            index,
            device_name,
            label,
            src_type: Box::new(src_type),
        }
    }
}
