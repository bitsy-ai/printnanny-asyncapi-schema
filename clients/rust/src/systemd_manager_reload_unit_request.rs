// SystemdManagerReloadUnitRequest represents a SystemdManagerReloadUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerReloadUnitRequest {
    #[serde(rename="unit_name")]
    pub unit_name: String,
}

impl SystemdManagerReloadUnitRequest {
    pub fn new(unit_name: String) -> SystemdManagerReloadUnitRequest {
        SystemdManagerReloadUnitRequest {
            unit_name,
        }
    }
}
