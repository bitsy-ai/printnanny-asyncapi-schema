// SettingsLoadReply represents a SettingsLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsLoadReply {
    #[serde(rename="content", skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(rename="format", skip_serializing_if = "Option::is_none")]
    format: Option<Box<crate::SettingsFormat>>,
    #[serde(rename="filename", skip_serializing_if = "Option::is_none")]
    filename: Option<Box<crate::SettingsFile>>,
    #[serde(rename="git_history", skip_serializing_if = "Option::is_none")]
    git_history: Option<Vec<crate::ReservedUnion>>,
    #[serde(rename="head_git_commit", skip_serializing_if = "Option::is_none")]
    head_git_commit: Option<String>,
}

impl SettingsLoadReply {
    pub fn new(content: Option<String>, format: Option<crate::SettingsFormat>, filename: Option<crate::SettingsFile>, git_history: Option<Vec<crate::ReservedUnion>>, head_git_commit: Option<String>) -> SettingsLoadReply {
        SettingsLoadReply {
            content,
            format: format.map(Box::new),
            filename: filename.map(Box::new),
            git_history,
            head_git_commit,
        }
    }
}
