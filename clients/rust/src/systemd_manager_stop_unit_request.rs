// SystemdManagerStopUnitRequest represents a SystemdManagerStopUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerStopUnitRequest {
    #[serde(rename="unit_name", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl SystemdManagerStopUnitRequest {
    pub fn new(unit_name: Option<String>) -> SystemdManagerStopUnitRequest {
        SystemdManagerStopUnitRequest {
            unit_name,
        }
    }
}
