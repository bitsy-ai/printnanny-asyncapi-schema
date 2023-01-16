// CameraSettings represents a CameraSettings model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CameraSettings {
    #[serde(rename="height")]
    pub height: i32,
    #[serde(rename="width")]
    pub width: i32,
    #[serde(rename="framerate")]
    pub framerate: f64,
    #[serde(rename="device_name")]
    pub device_name: String,
    #[serde(rename="label")]
    pub label: String,
}

impl CameraSettings {
    pub fn new(height: i32, width: i32, framerate: f64, device_name: String, label: String) -> CameraSettings {
        CameraSettings {
            height,
            width,
            framerate,
            device_name,
            label,
        }
    }
}
