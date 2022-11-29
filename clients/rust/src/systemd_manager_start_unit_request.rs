// SystemdManagerStartUnitRequest represents a SystemdManagerStartUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerStartUnitRequest {
    #[serde(rename="unit_name", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl SystemdManagerStartUnitRequest {
    pub fn new(unit_name: Option<String>) -> SystemdManagerStartUnitRequest {
        SystemdManagerStartUnitRequest {
            unit_name,
        }
    }
}
