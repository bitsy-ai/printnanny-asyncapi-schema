// NetworkInterfaceAddress represents a NetworkInterfaceAddress model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NetworkInterfaceAddress {
    #[serde(rename="interface_name")]
    pub interface_name: String,
    #[serde(rename="flags")]
    pub flags: Vec<String>,
    #[serde(rename="address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename="netmask", skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    #[serde(rename="broadcast", skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<String>,
    #[serde(rename="destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

impl NetworkInterfaceAddress {
    pub fn new(interface_name: String, flags: Vec<String>, address: Option<String>, netmask: Option<String>, broadcast: Option<String>, destination: Option<String>) -> NetworkInterfaceAddress {
        NetworkInterfaceAddress {
            interface_name,
            flags,
            address,
            netmask,
            broadcast,
            destination,
        }
    }
}
