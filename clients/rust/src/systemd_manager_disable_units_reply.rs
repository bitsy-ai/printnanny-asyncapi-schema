// SystemdManagerDisableUnitsReply represents a SystemdManagerDisableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerDisableUnitsReply {
    #[serde(rename="changes")]
    pub changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerDisableUnitsReply {
    pub fn new(changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerDisableUnitsReply {
        SystemdManagerDisableUnitsReply {
            changes,
        }
    }
}
