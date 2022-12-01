// SettingsRevertRequest represents a SettingsRevertRequest model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SettingsRevertRequest {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="files")]
    pub files: serde_json::Value,
    #[serde(rename="git_commit")]
    pub git_commit: String,
}

impl SettingsRevertRequest {
    pub fn new(app: crate::SettingsApp, files: serde_json::Value, git_commit: String) -> SettingsRevertRequest {
        SettingsRevertRequest {
            app: Box::new(app),
            files,
            git_commit,
        }
    }
}
