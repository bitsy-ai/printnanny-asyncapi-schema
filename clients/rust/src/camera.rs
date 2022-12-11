// Camera represents a Camera model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Camera {
    #[serde(rename="index")]
    pub index: i32,
    #[serde(rename="device_name")]
    pub device_name: String,
    #[serde(rename="label")]
    pub label: String,
}

impl Camera {
    pub fn new(index: i32, device_name: String, label: String) -> Camera {
        Camera {
            index,
            device_name,
            label,
        }
    }
}
