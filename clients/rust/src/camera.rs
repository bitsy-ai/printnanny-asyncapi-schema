// Camera represents a Camera model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Camera {
    #[serde(rename="index")]
    pub index: i32,
    #[serde(rename="device_name")]
    pub device_name: String,
    #[serde(rename="label")]
    pub label: String,
    #[serde(rename="video_src")]
    pub video_src: Box<crate::VideoSourceType>,
}

impl Camera {
    pub fn new(index: i32, device_name: String, label: String, video_src: crate::VideoSourceType) -> Camera {
        Camera {
            index,
            device_name,
            label,
            video_src: Box::new(video_src),
        }
    }
}
