// SettingsFileApplyRequest represents a SettingsFileApplyRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsFileApplyRequest {
    #[serde(rename = "file")]
    pub file: Box<crate::SettingsFile>,
    #[serde(rename = "git_head_commit")]
    pub git_head_commit: String,
    #[serde(rename = "git_commit_msg")]
    pub git_commit_msg: String,
}

impl SettingsFileApplyRequest {
    pub fn new(
        file: crate::SettingsFile,
        git_head_commit: String,
        git_commit_msg: String,
    ) -> SettingsFileApplyRequest {
        SettingsFileApplyRequest {
            file: Box::new(file),
            git_head_commit,
            git_commit_msg,
        }
    }
}
