// SettingsFile represents a SettingsFile model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SettingsFile {
    #[serde(rename="octoprint.yaml")]
    OctoprintDotYaml,
    #[serde(rename="klipper.cfg")]
    KlipperDotCfg,
    #[serde(rename="moonraker.conf")]
    MoonrakerDotConf,
    #[serde(rename="printnanny.toml")]
    PrintnannyDotToml,
}
impl Default for SettingsFile {
    fn default() -> SettingsFile {
        SettingsFile::OctoprintDotYaml
    }
}