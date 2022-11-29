// SettingsApplyReply represents a SettingsApplyReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsApplyReply {
    #[serde(rename="format")]
    pub format: Box<crate::SettingsFormat>,
    #[serde(rename="filename")]
    pub filename: Box<crate::SettingsFile>,
    #[serde(rename="content")]
    pub content: String,
    #[serde(rename="head_git_commit")]
    pub head_git_commit: String,
    #[serde(rename="git_history")]
    pub git_history: Vec<crate::GitCommit>,
}

impl SettingsApplyReply {
    pub fn new(format: crate::SettingsFormat, filename: crate::SettingsFile, content: String, head_git_commit: String, git_history: Vec<crate::GitCommit>) -> SettingsApplyReply {
        SettingsApplyReply {
            format: Box::new(format),
            filename: Box::new(filename),
            content,
            head_git_commit,
            git_history,
        }
    }
}
