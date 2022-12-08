// SettingsApplyRequest represents a SettingsApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsApplyRequest {
    #[serde(rename="files")]
    pub files: Vec<crate::SettingsFile>,
    #[serde(rename="git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename="git_commit_msg")]
    pub git_commit_msg: String,
}

impl SettingsApplyRequest {
    pub fn new(files: Vec<crate::SettingsFile>, git_head_commit: String, git_commit_msg: String) -> SettingsApplyRequest {
        SettingsApplyRequest {
            files,
            git_head_commit,
            git_commit_msg,
        }
    }
}
