// SystemdManagerUnitRequest represents a SystemdManagerUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerUnitRequest {
    #[serde(rename="unit_name", skip_serializing_if = "Option::is_none")]
    unit_name: Option<String>,
}

impl SystemdManagerUnitRequest {
    pub fn new(unit_name: Option<String>) -> SystemdManagerUnitRequest {
        SystemdManagerUnitRequest {
            unit_name,
        }
    }
}
