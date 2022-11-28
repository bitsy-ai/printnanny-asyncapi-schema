// SystemdManagerChangeUnitReply represents a SystemdManagerChangeUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerChangeUnitReply {
    #[serde(rename="request")]
    request: Box<crate::SystemdManagerChangeUnitRequest>,
    #[serde(rename="changes")]
    changes: Vec<crate::SystemdUnitChange>,
}

impl SystemdManagerChangeUnitReply {
    pub fn new(request: crate::SystemdManagerChangeUnitRequest, changes: Vec<crate::SystemdUnitChange>) -> SystemdManagerChangeUnitReply {
        SystemdManagerChangeUnitReply {
            request: Box::new(request),
            changes,
        }
    }
}
