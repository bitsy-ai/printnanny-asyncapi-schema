// SettingsLoadRequest represents a SettingsLoadRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SettingsLoadRequest {
    #[serde(rename="app")]
    pub app: Box<crate::SettingsApp>,
}

impl SettingsLoadRequest {
    pub fn new(app: crate::SettingsApp) -> SettingsLoadRequest {
        SettingsLoadRequest {
            app: Box::new(app),
        }
    }
}
