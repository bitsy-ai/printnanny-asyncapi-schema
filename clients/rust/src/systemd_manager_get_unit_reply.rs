// SystemdManagerGetUnitReply represents a SystemdManagerGetUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitReply {
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerGetUnitReply {
    pub fn new(unit: crate::SystemdUnit) -> SystemdManagerGetUnitReply {
        SystemdManagerGetUnitReply {
            unit: Box::new(unit),
        }
    }
}
