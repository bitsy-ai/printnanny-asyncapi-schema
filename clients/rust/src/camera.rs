// Camera represents a Camera model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Camera {
    #[serde(rename="index")]
    pub index: i32,
    #[serde(rename="name")]
    pub name: String,
    #[serde(rename="label")]
    pub label: String,
}

impl Camera {
    pub fn new(index: i32, name: String, label: String) -> Camera {
        Camera {
            index,
            name,
            label,
        }
    }
}
