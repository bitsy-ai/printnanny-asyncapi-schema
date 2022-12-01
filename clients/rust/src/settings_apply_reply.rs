// SettingsApplyReply represents a SettingsApplyReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SettingsApplyReply {
    #[serde(rename="app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<crate::SettingsApp>>,
    #[serde(rename="files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Serde_json::Value>>,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_history")]
    pub git_history: Vec<crate::GitCommit>,
}

impl SettingsApplyReply {
    pub fn new(app: Option<crate::SettingsApp>, files: Option<Vec<Serde_json::Value>>, git_head_commit: String, git_history: Vec<crate::GitCommit>) -> SettingsApplyReply {
        SettingsApplyReply {
            app: app.map(Box::new),
            files,
            git_head_commit,
            git_history,
        }
    }
}
