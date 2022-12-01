// SettingsApplyRequest represents a SettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SettingsApplyRequest {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="files")]
    pub files: serde_json::Value,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_commit_msg")]
    pub git_commit_msg: String,
}

impl SettingsApplyRequest {
    pub fn new(app: crate::SettingsApp, files: serde_json::Value, git_head_commit: String, git_commit_msg: String) -> SettingsApplyRequest {
        SettingsApplyRequest {
            app: Box::new(app),
            files,
            git_head_commit,
            git_commit_msg,
        }
    }
}
