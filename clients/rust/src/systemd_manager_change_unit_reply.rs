// SystemdManagerChangeUnitReply represents a SystemdManagerChangeUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerChangeUnitReply {
    #[serde(rename="request")]
    request: Box<crate::SystemdManagerChangeUnitRequest>,
    #[serde(rename="changes")]
    changes: Vec<crate::ReservedUnion>,
}

impl SystemdManagerChangeUnitReply {
    pub fn new(request: crate::SystemdManagerChangeUnitRequest, changes: Vec<crate::ReservedUnion>) -> SystemdManagerChangeUnitReply {
        SystemdManagerChangeUnitReply {
        Box::new(request),
        changes,
        }
    }
}
