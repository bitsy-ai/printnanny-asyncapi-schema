// SystemdManagerStartUnitRequest represents a SystemdManagerStartUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerStartUnitRequest {
    #[serde(rename="unit_name")]
    pub unit_name: String,
}

impl SystemdManagerStartUnitRequest {
    pub fn new(unit_name: String) -> SystemdManagerStartUnitRequest {
        SystemdManagerStartUnitRequest {
            unit_name,
        }
    }
}
