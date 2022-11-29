// SystemdManagerEnableUnitsReply represents a SystemdManagerEnableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerEnableUnitsReply {
    #[serde(rename="request")]
    pub request: serde_json::Value,
    #[serde(rename="changes")]
    pub changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerEnableUnitsReply {
    pub fn new(request: serde_json::Value, changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerEnableUnitsReply {
        SystemdManagerEnableUnitsReply {
            request,
            changes,
        }
    }
}
