// SystemdManagerStopUnitRequest represents a SystemdManagerStopUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerStopUnitRequest {
    #[serde(rename="unit_name")]
    pub unit_name: String,
}

impl SystemdManagerStopUnitRequest {
    pub fn new(unit_name: String) -> SystemdManagerStopUnitRequest {
        SystemdManagerStopUnitRequest {
            unit_name,
        }
    }
}
