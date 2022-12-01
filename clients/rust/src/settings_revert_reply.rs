// SettingsRevertReply represents a SettingsRevertReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SettingsRevertReply {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="files")]
    pub files: serde_json::Value,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_history")]
    pub git_history: Vec<crate::GitCommit>,
}

impl SettingsRevertReply {
    pub fn new(app: crate::SettingsApp, files: serde_json::Value, git_head_commit: String, git_history: Vec<crate::GitCommit>) -> SettingsRevertReply {
        SettingsRevertReply {
            app: Box::new(app),
            files,
            git_head_commit,
            git_history,
        }
    }
}
