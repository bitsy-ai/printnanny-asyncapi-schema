// SystemdManagerEnableUnitsReply represents a SystemdManagerEnableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerEnableUnitsReply {
    #[serde(rename="changes")]
    pub changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerEnableUnitsReply {
    pub fn new(changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerEnableUnitsReply {
        SystemdManagerEnableUnitsReply {
            changes,
        }
    }
}
