// SystemdManagerRestartUnitRequest represents a SystemdManagerRestartUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerRestartUnitRequest {
    #[serde(rename="unit_name")]
    pub unit_name: String,
}

impl SystemdManagerRestartUnitRequest {
    pub fn new(unit_name: String) -> SystemdManagerRestartUnitRequest {
        SystemdManagerRestartUnitRequest {
            unit_name,
        }
    }
}
