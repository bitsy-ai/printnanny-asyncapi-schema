// SettingsLoadRequest represents a SettingsLoadRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsLoadRequest {
    #[serde(rename="format", skip_serializing_if = "Option::is_none")]
    format: Option<Box<crate::SettingsFormat>>,
    #[serde(rename="filename", skip_serializing_if = "Option::is_none")]
    filename: Option<Box<crate::SettingsFile>>,
}

impl SettingsLoadRequest {
    pub fn new(format: Option<crate::SettingsFormat>, filename: Option<crate::SettingsFile>) -> SettingsLoadRequest {
        SettingsLoadRequest {
            format: format.map(Box::new),
            filename: filename.map(Box::new),
        }
    }
}
