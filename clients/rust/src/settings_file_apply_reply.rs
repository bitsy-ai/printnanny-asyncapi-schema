// SettingsFileApplyReply represents a SettingsFileApplyReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsFileApplyReply {
    #[serde(rename="file")]
    pub file: Box<crate::SettingsFile>,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_history")]
    pub git_history: Vec<crate::GitCommit>,
}

impl SettingsFileApplyReply {
    pub fn new(file: crate::SettingsFile, git_head_commit: String, git_history: Vec<crate::GitCommit>) -> SettingsFileApplyReply {
        SettingsFileApplyReply {
            file: Box::new(file),
            git_head_commit,
            git_history,
        }
    }
}
