// SystemdManagerDisableUnitsReply represents a SystemdManagerDisableUnitsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerDisableUnitsReply {
    #[serde(rename="request")]
    pub request: Box<crate::SystemdManagerUnitFilesRequest>,
    #[serde(rename="changes")]
    pub changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerDisableUnitsReply {
    pub fn new(request: crate::SystemdManagerUnitFilesRequest, changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerDisableUnitsReply {
        SystemdManagerDisableUnitsReply {
            request: Box::new(request),
            changes,
        }
    }
}
