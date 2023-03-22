// SettingsFormat represents a SettingsFormat model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SettingsFormat {
    #[serde(rename="json")]
    Json,
    #[serde(rename="toml")]
    Toml,
    #[serde(rename="yaml")]
    Yaml,
    #[serde(rename="ini")]
    Ini,
}
impl Default for SettingsFormat {
    fn default() -> SettingsFormat {
        SettingsFormat::Json
    }
}