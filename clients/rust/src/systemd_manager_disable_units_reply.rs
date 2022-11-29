// SystemdManagerDisableUnitsReply represents a SystemdManagerDisableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdManagerDisableUnitsReply {
    #[serde(rename="request")]
    pub request: serde_json::Value,
    #[serde(rename="changes")]
    pub changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerDisableUnitsReply {
    pub fn new(request: serde_json::Value, changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerDisableUnitsReply {
        SystemdManagerDisableUnitsReply {
            request,
            changes,
        }
    }
}
