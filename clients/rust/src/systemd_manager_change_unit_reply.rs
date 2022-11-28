// SystemdManagerChangeUnitReply represents a SystemdManagerChangeUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerChangeUnitReply {
    #[serde(rename="request")]
    request: Box<crate::AnonymousSchema12>,
    #[serde(rename="changes")]
    changes: Vec<crate::ReservedUnion>,
}

impl SystemdManagerChangeUnitReply {
    pub fn new(request: crate::AnonymousSchema12, changes: Vec<crate::ReservedUnion>) -> SystemdManagerChangeUnitReply {
        SystemdManagerChangeUnitReply {
        Box::new(request),
        changes,
        }
    }
}
