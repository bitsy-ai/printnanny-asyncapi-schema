// SystemdManagerGetUnitRequest represents a SystemdManagerGetUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitRequest {
    #[serde(rename="unit_name", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl SystemdManagerGetUnitRequest {
    pub fn new(unit_name: Option<String>) -> SystemdManagerGetUnitRequest {
        SystemdManagerGetUnitRequest {
            unit_name,
        }
    }
}
