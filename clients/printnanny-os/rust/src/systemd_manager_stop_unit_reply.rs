// SystemdManagerStopUnitReply represents a SystemdManagerStopUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerStopUnitReply {
    #[serde(rename="job")]
    pub job: String,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerStopUnitReply {
    pub fn new(job: String, unit: crate::SystemdUnit) -> SystemdManagerStopUnitReply {
        SystemdManagerStopUnitReply {
            job,
            unit: Box::new(unit),
        }
    }
}
