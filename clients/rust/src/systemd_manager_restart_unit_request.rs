// SystemdManagerRestartUnitRequest represents a SystemdManagerRestartUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerRestartUnitRequest {
    #[serde(rename="unit_name", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl SystemdManagerRestartUnitRequest {
    pub fn new(unit_name: Option<String>) -> SystemdManagerRestartUnitRequest {
        SystemdManagerRestartUnitRequest {
            unit_name,
        }
    }
}
