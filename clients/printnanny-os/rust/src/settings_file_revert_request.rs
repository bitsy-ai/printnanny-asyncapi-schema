// SettingsFileRevertRequest represents a SettingsFileRevertRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsFileRevertRequest {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
    #[serde(rename="files")]
    pub files: Vec<crate::SettingsFile>,
    #[serde(rename="git_commit")]
    pub git_commit: String,
}

impl SettingsFileRevertRequest {
    pub fn new(app: crate::SettingsApp, files: Vec<crate::SettingsFile>, git_commit: String) -> SettingsFileRevertRequest {
        SettingsFileRevertRequest {
            app: Box::new(app),
            files,
            git_commit,
        }
    }
}
