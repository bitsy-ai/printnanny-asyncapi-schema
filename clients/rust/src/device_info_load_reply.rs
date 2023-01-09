// DeviceInfoLoadReply represents a DeviceInfoLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeviceInfoLoadReply {
    #[serde(rename="issue")]
    pub issue: String,
    #[serde(rename="os_release")]
    pub os_release: String,
    #[serde(rename="printnanny_cli_version")]
    pub printnanny_cli_version: String,
    #[serde(rename="ifaddrs", skip_serializing_if = "Option::is_none")]
    pub ifaddrs: Option<String>,
}

impl DeviceInfoLoadReply {
    pub fn new(issue: String, os_release: String, printnanny_cli_version: String, ifaddrs: Option<String>) -> DeviceInfoLoadReply {
        DeviceInfoLoadReply {
            issue,
            os_release,
            printnanny_cli_version,
            ifaddrs,
        }
    }
}
