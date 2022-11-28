// SystemdManagerGetUnitReply represents a SystemdManagerGetUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitReply {
    #[serde(rename="request")]
    request: Box<crate::SystemdManagerGetUnitRequest>,
    #[serde(rename="unit")]
    unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerGetUnitReply {
    pub fn new(request: crate::SystemdManagerGetUnitRequest, unit: crate::SystemdUnit) -> SystemdManagerGetUnitReply {
        SystemdManagerGetUnitReply {
            request: Box::new(request),
            unit: Box::new(unit),
        }
    }
}
