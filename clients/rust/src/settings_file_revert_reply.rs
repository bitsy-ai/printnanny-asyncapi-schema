// SettingsFileRevertReply represents a SettingsFileRevertReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsFileRevertReply {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="files")]
    pub files: Vec<crate::SettingsFile>,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_history")]
    pub git_history: Vec<crate::GitCommit>,
}

impl SettingsFileRevertReply {
    pub fn new(app: crate::SettingsApp, files: Vec<crate::SettingsFile>, git_head_commit: String, git_history: Vec<crate::GitCommit>) -> SettingsFileRevertReply {
        SettingsFileRevertReply {
            app: Box::new(app),
            files,
            git_head_commit,
            git_history,
        }
    }
}
