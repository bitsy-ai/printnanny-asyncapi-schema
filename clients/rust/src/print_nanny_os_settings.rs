// PrintNannyOsSettings represents a PrintNannyOsSettings model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyOsSettings {
    #[serde(rename = "cloud")]
    pub cloud: Box<crate::PrintNannyCloudApiConfig>,
    #[serde(rename = "video")]
    pub video: Box<crate::VideoStreamSettings>,
    #[serde(rename = "git")]
    pub git: Box<crate::GitSettings>,
    #[serde(rename = "paths")]
    pub paths: Box<crate::PathSettings>,
    #[serde(rename = "klipper")]
    pub klipper: Box<crate::KlipperSettings>,
    #[serde(rename = "mainsail")]
    pub mainsail: Box<crate::MainsailSettings>,
    #[serde(rename = "moonraker")]
    pub moonraker: Box<crate::MoonrakerSettings>,
    #[serde(rename = "octoprint")]
    pub octoprint: Box<crate::OctoPrintSettings>,
}

impl PrintNannyOsSettings {
    pub fn new(
        cloud: crate::PrintNannyCloudApiConfig,
        video: crate::VideoStreamSettings,
        git: crate::GitSettings,
        paths: crate::PathSettings,
        klipper: crate::KlipperSettings,
        mainsail: crate::MainsailSettings,
        moonraker: crate::MoonrakerSettings,
        octoprint: crate::OctoPrintSettings,
    ) -> PrintNannyOsSettings {
        PrintNannyOsSettings {
            cloud: Box::new(cloud),
            video: Box::new(video),
            git: Box::new(git),
            paths: Box::new(paths),
            klipper: Box::new(klipper),
            mainsail: Box::new(mainsail),
            moonraker: Box::new(moonraker),
            octoprint: Box::new(octoprint),
        }
    }
}
