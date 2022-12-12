// SystemdManagerEnableUnitsReply represents a SystemdManagerEnableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerEnableUnitsReply {
    #[serde(rename="request")]
    pub request: Box<crate::SystemdManagerUnitFilesRequest>,
    #[serde(rename="changes")]
    pub changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerEnableUnitsReply {
    pub fn new(request: crate::SystemdManagerUnitFilesRequest, changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerEnableUnitsReply {
        SystemdManagerEnableUnitsReply {
            request: Box::new(request),
            changes,
        }
    }
}
