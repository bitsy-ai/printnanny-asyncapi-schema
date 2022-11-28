// SystemdManagerGetUnitRequest represents a SystemdManagerGetUnitRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitRequest {
    #[serde(rename="name")]
    name: String,
}

impl SystemdManagerGetUnitRequest {
    pub fn new(name: String) -> SystemdManagerGetUnitRequest {
        SystemdManagerGetUnitRequest {
            name,
        }
    }
}
