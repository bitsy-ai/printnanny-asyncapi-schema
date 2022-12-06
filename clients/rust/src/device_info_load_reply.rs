// DeviceInfoLoadReply represents a DeviceInfoLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeviceInfoLoadReply {
    #[serde(rename="issue", skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[serde(rename="os_release", skip_serializing_if = "Option::is_none")]
    pub os_release: Option<String>,
    #[serde(rename="printnanny_cli_version", skip_serializing_if = "Option::is_none")]
    pub printnanny_cli_version: Option<String>,
}

impl DeviceInfoLoadReply {
    pub fn new(issue: Option<String>, os_release: Option<String>, printnanny_cli_version: Option<String>) -> DeviceInfoLoadReply {
        DeviceInfoLoadReply {
            issue,
            os_release,
            printnanny_cli_version,
        }
    }
}
