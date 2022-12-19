// SettingsFileLoadReply represents a SettingsFileLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsFileLoadReply {
    #[serde(rename = "files")]
    pub files: Vec<crate::SettingsFile>,
    #[serde(rename = "git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename = "git_history")]
    pub git_history: Vec<crate::GitCommit>,
}

impl SettingsFileLoadReply {
    pub fn new(
        files: Vec<crate::SettingsFile>,
        git_head_commit: String,
        git_history: Vec<crate::GitCommit>,
    ) -> SettingsFileLoadReply {
        SettingsFileLoadReply {
            files,
            git_head_commit,
            git_history,
        }
    }
}
