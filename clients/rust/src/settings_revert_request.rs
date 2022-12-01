// SettingsRevertRequest represents a SettingsRevertRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsRevertRequest {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="files")]
    pub files: Vec<crate::SettingsFile>,
    #[serde(rename="git_commit")]
    pub git_commit: String,
}

impl SettingsRevertRequest {
    pub fn new(app: crate::SettingsApp, files: Vec<crate::SettingsFile>, git_commit: String) -> SettingsRevertRequest {
        SettingsRevertRequest {
            app: Box::new(app),
            files,
            git_commit,
        }
    }
}
