// DeviceInfoLoadReply represents a DeviceInfoLoadReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeviceInfoLoadReply {
    #[serde(rename="issue")]
    pub issue: String,
    #[serde(rename="os_release")]
    pub os_release: String,
    #[serde(rename="printnanny_cli_version")]
    pub printnanny_cli_version: String,
    #[serde(rename="tailscale_address_ipv4", skip_serializing_if = "Option::is_none")]
    pub tailscale_address_ipv4: Option<String>,
    #[serde(rename="tailscale_address_ipv6", skip_serializing_if = "Option::is_none")]
    pub tailscale_address_ipv6: Option<String>,
}

impl DeviceInfoLoadReply {
    pub fn new(issue: String, os_release: String, printnanny_cli_version: String, tailscale_address_ipv4: Option<String>, tailscale_address_ipv6: Option<String>) -> DeviceInfoLoadReply {
        DeviceInfoLoadReply {
            issue,
            os_release,
            printnanny_cli_version,
            tailscale_address_ipv4,
            tailscale_address_ipv6,
        }
    }
}
