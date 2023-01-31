// CameraSettings represents a CameraSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CameraSettings {
    #[serde(rename="colorimetry")]
    pub colorimetry: String,
    #[serde(rename="height")]
    pub height: i32,
    #[serde(rename="width")]
    pub width: i32,
    #[serde(rename="framerate_n")]
    pub framerate_n: i32,
    #[serde(rename="framerate_d")]
    pub framerate_d: i32,
    #[serde(rename="format")]
    pub format: String,
    #[serde(rename="device_name")]
    pub device_name: String,
    #[serde(rename="label")]
    pub label: String,
}

impl CameraSettings {
    pub fn new(colorimetry: String, height: i32, width: i32, framerate_n: i32, framerate_d: i32, format: String, device_name: String, label: String) -> CameraSettings {
        CameraSettings {
            colorimetry,
            height,
            width,
            framerate_n,
            framerate_d,
            format,
            device_name,
            label,
        }
    }
}
