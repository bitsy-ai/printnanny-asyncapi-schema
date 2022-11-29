// SystemdManagerReloadUnitRequest represents a SystemdManagerReloadUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerReloadUnitRequest {
    #[serde(rename="unit_name", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl SystemdManagerReloadUnitRequest {
    pub fn new(unit_name: Option<String>) -> SystemdManagerReloadUnitRequest {
        SystemdManagerReloadUnitRequest {
            unit_name,
        }
    }
}
