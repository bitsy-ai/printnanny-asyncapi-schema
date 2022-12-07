// SystemdManagerGetUnitFileStateRequest represents a SystemdManagerGetUnitFileStateRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitFileStateRequest {
    #[serde(rename="unit_name")]
    pub unit_name: String,
}

impl SystemdManagerGetUnitFileStateRequest {
    pub fn new(unit_name: String) -> SystemdManagerGetUnitFileStateRequest {
        SystemdManagerGetUnitFileStateRequest {
            unit_name,
        }
    }
}
