// MoonrakerSettings represents a MoonrakerSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MoonrakerSettings {
    #[serde(rename="enabled")]
    pub enabled: bool,
    #[serde(rename="install_dir")]
    pub install_dir: String,
    #[serde(rename="settings_file")]
    pub settings_file: String,
    #[serde(rename="settings_format")]
    pub settings_format: Box<crate::SettingsFormat>,
    #[serde(rename="venv")]
    pub venv: String,
}

impl MoonrakerSettings {
    pub fn new(enabled: bool, install_dir: String, settings_file: String, settings_format: crate::SettingsFormat, venv: String) -> MoonrakerSettings {
        MoonrakerSettings {
            enabled,
            install_dir,
            settings_file,
            settings_format: Box::new(settings_format),
            venv,
        }
    }
}
