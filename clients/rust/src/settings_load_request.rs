// SettingsLoadRequest represents a SettingsLoadRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsLoadRequest {
    #[serde(rename="format")]
    format: Box<crate::SettingsFormat>,
    #[serde(rename="filename")]
    filename: Box<crate::SettingsFile>,
}

impl SettingsLoadRequest {
    pub fn new(format: crate::SettingsFormat, filename: crate::SettingsFile) -> SettingsLoadRequest {
        SettingsLoadRequest {
            format: Box::new(format),
            filename: Box::new(filename),
        }
    }
}
