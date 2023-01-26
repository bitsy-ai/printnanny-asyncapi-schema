// OctoPrintSettings represents a OctoPrintSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintSettings {
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

impl OctoPrintSettings {
    pub fn new(enabled: bool, install_dir: String, settings_file: String, settings_format: crate::SettingsFormat, venv: String) -> OctoPrintSettings {
        OctoPrintSettings {
            enabled,
            install_dir,
            settings_file,
            settings_format: Box::new(settings_format),
            venv,
        }
    }
}
