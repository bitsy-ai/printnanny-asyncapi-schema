// GcodeFile represents a GcodeFile model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GcodeFile {
    #[serde(rename="file_name")]
    pub file_name: String,
    #[serde(rename="display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename="file_path")]
    pub file_path: String,
    #[serde(rename="origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename="size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename="timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl GcodeFile {
    pub fn new(file_name: String, display: Option<String>, file_path: String, origin: Option<String>, size: Option<i64>, timestamp: Option<i64>) -> GcodeFile {
        GcodeFile {
            file_name,
            display,
            file_path,
            origin,
            size,
            timestamp,
        }
    }
}
