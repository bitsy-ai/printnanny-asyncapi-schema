// SettingsApplyReply represents a SettingsApplyReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsApplyReply {
    #[serde(rename="format", skip_serializing_if = "Option::is_none")]
    format: Option<Box<crate::SettingsFormat>>,
    #[serde(rename="filename", skip_serializing_if = "Option::is_none")]
    filename: Option<Box<crate::SettingsFile>>,
    #[serde(rename="content", skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(rename="head_git_commit", skip_serializing_if = "Option::is_none")]
    head_git_commit: Option<String>,
}

impl SettingsApplyReply {
    pub fn new(format: Option<crate::SettingsFormat>, filename: Option<crate::SettingsFile>, content: Option<String>, head_git_commit: Option<String>) -> SettingsApplyReply {
        SettingsApplyReply {
            format: format.map(Box::new),
            filename: filename.map(Box::new),
            content,
            head_git_commit,
        }
    }
}
