// MainsailSettings represents a MainsailSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MainsailSettings {
    #[serde(rename="enabled")]
    pub enabled: bool,
    #[serde(rename="install_dir")]
    pub install_dir: String,
}

impl MainsailSettings {
    pub fn new(enabled: bool, install_dir: String) -> MainsailSettings {
        MainsailSettings {
            enabled,
            install_dir,
        }
    }
}
