// SettingsApp represents a SettingsApp model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SettingsApp {
    #[serde(rename="octoprint")]
    Octoprint,
    #[serde(rename="klipper")]
    Klipper,
    #[serde(rename="moonraker")]
    Moonraker,
    #[serde(rename="printnanny")]
    Printnanny,
}
impl Default for SettingsApp {
    fn default() -> SettingsApp {
        SettingsApp::Octoprint
    }
}