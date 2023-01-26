// SettingsFile represents a SettingsFile model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsFile {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="content")]
    pub content: String,
    #[serde(rename="file_name")]
    pub file_name: String,
    #[serde(rename="file_format")]
    pub file_format: Box<crate::SettingsFormat>,
}

impl SettingsFile {
    pub fn new(app: crate::SettingsApp, content: String, file_name: String, file_format: crate::SettingsFormat) -> SettingsFile {
        SettingsFile {
            app: Box::new(app),
            content,
            file_name,
            file_format: Box::new(file_format),
        }
    }
}
