// SettingsApplyRequest represents a SettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsApplyRequest {
    #[serde(rename="format")]
    pub format: Box<crate::SettingsFormat>,
    #[serde(rename="filename")]
    pub filename: Box<crate::SettingsFile>,
    #[serde(rename="content")]
    pub content: String,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_commit_msg")]
    pub git_commit_msg: String,
}

impl SettingsApplyRequest {
    pub fn new(format: crate::SettingsFormat, filename: crate::SettingsFile, content: String, git_head_commit: String, git_commit_msg: String) -> SettingsApplyRequest {
        SettingsApplyRequest {
            format: Box::new(format),
            filename: Box::new(filename),
            content,
            git_head_commit,
            git_commit_msg,
        }
    }
}
