// SettingsApplyRequest represents a SettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsApplyRequest {
    #[serde(rename="format")]
    format: Box<crate::SettingsFormat>,
    #[serde(rename="filename")]
    filename: Box<crate::SettingsFile>,
    #[serde(rename="content")]
    content: String,
    #[serde(rename="head_git_commit")]
    head_git_commit: String,
}

impl SettingsApplyRequest {
    pub fn new(format: crate::SettingsFormat, filename: crate::SettingsFile, content: String, head_git_commit: String) -> SettingsApplyRequest {
        SettingsApplyRequest {
            format: Box::new(format),
            filename: Box::new(filename),
            content,
            head_git_commit,
        }
    }
}
