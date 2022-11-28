// SystemdManagerEnableUnitsReply represents a SystemdManagerEnableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerEnableUnitsReply {
    #[serde(rename="request")]
    request: Box<crate::SystemdManagerEnableUnitsRequest>,
    #[serde(rename="changes")]
    changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerEnableUnitsReply {
    pub fn new(request: crate::SystemdManagerEnableUnitsRequest, changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerEnableUnitsReply {
        SystemdManagerEnableUnitsReply {
            request: Box::new(request),
            changes,
        }
    }
}
