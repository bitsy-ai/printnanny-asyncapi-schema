// SystemdManagerGetUnitRequest represents a SystemdManagerGetUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitRequest {
    #[serde(rename="unit_name")]
    pub unit_name: String,
}

impl SystemdManagerGetUnitRequest {
    pub fn new(unit_name: String) -> SystemdManagerGetUnitRequest {
        SystemdManagerGetUnitRequest {
            unit_name,
        }
    }
}
