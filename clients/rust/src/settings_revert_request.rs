// SettingsRevertRequest represents a SettingsRevertRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsRevertRequest {
    #[serde(rename="git_commit")]
    git_commit: String,
}

impl SettingsRevertRequest {
    pub fn new(git_commit: String) -> SettingsRevertRequest {
        SettingsRevertRequest {
            git_commit,
        }
    }
}
