// SettingsApplyRequest represents a SettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsApplyRequest {
    #[serde(rename="format", skip_serializing_if = "Option::is_none")]
    format: Option<Box<crate::SettingsFormat>>,
    #[serde(rename="filename", skip_serializing_if = "Option::is_none")]
    filename: Option<Box<crate::SettingsFile>>,
    #[serde(rename="content", skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(rename="head_git_commit", skip_serializing_if = "Option::is_none")]
    head_git_commit: Option<String>,
}

impl SettingsApplyRequest {
    pub fn new(format: Option<crate::SettingsFormat>, filename: Option<crate::SettingsFile>, content: Option<String>, head_git_commit: Option<String>) -> SettingsApplyRequest {
        SettingsApplyRequest {
            format: format.map(Box::new),
            filename: filename.map(Box::new),
            content,
            head_git_commit,
        }
    }
}
