// PrintNannyOsSettings represents a PrintNannyOsSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyOsSettings {
    #[serde(rename="camera", skip_serializing_if = "Option::is_none")]
    pub camera: Option<Box<crate::PrintNannyCameraSettings>>,
    #[serde(rename="cloud", skip_serializing_if = "Option::is_none")]
    pub cloud: Option<Box<crate::PrintNannyCloudApiConfig>>,
    #[serde(rename="git", skip_serializing_if = "Option::is_none")]
    pub git: Option<Box<crate::GitSettings>>,
    #[serde(rename="paths", skip_serializing_if = "Option::is_none")]
    pub paths: Option<Box<crate::PathSettings>>,
    #[serde(rename="klipper", skip_serializing_if = "Option::is_none")]
    pub klipper: Option<Box<crate::KlipperSettings>>,
    #[serde(rename="mainsail", skip_serializing_if = "Option::is_none")]
    pub mainsail: Option<Box<crate::MainsailSettings>>,
    #[serde(rename="moonraker", skip_serializing_if = "Option::is_none")]
    pub moonraker: Option<Box<crate::MoonrakerSettings>>,
    #[serde(rename="octoprint", skip_serializing_if = "Option::is_none")]
    pub octoprint: Option<Box<crate::OctoPrintSettings>>,
}

impl PrintNannyOsSettings {
    pub fn new(camera: Option<crate::PrintNannyCameraSettings>, cloud: Option<crate::PrintNannyCloudApiConfig>, git: Option<crate::GitSettings>, paths: Option<crate::PathSettings>, klipper: Option<crate::KlipperSettings>, mainsail: Option<crate::MainsailSettings>, moonraker: Option<crate::MoonrakerSettings>, octoprint: Option<crate::OctoPrintSettings>) -> PrintNannyOsSettings {
        PrintNannyOsSettings {
            camera: camera.map(Box::new),
            cloud: cloud.map(Box::new),
            git: git.map(Box::new),
            paths: paths.map(Box::new),
            klipper: klipper.map(Box::new),
            mainsail: mainsail.map(Box::new),
            moonraker: moonraker.map(Box::new),
            octoprint: octoprint.map(Box::new),
        }
    }
}
