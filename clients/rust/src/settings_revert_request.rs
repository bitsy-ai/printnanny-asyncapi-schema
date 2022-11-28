// SettingsRevertRequest represents a SettingsRevertRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsRevertRequest {
    #[serde(rename="git_commit", skip_serializing_if = "Option::is_none")]
    git_commit: Option<String>,
}

impl SettingsRevertRequest {
    pub fn new(git_commit: Option<String>) -> SettingsRevertRequest {
        SettingsRevertRequest {
            git_commit,
        }
    }
}
