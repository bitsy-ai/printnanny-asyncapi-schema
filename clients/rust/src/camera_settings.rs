// CameraSettings represents a CameraSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraSettings {
    #[serde(rename="height")]
    pub height: i32,
    #[serde(rename="width")]
    pub width: i32,
    #[serde(rename="framerate")]
    pub framerate: i32,
    #[serde(rename="format")]
    pub format: String,
    #[serde(rename="device_name")]
    pub device_name: String,
    #[serde(rename="label")]
    pub label: String,
}

impl CameraSettings {
    pub fn new(height: i32, width: i32, framerate: i32, format: String, device_name: String, label: String) -> CameraSettings {
        CameraSettings {
            height,
            width,
            framerate,
            format,
            device_name,
            label,
        }
    }
}
