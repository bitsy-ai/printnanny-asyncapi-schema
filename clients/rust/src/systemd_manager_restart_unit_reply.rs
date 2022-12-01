// SystemdManagerRestartUnitReply represents a SystemdManagerRestartUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerRestartUnitReply {
    #[serde(rename="job")]
    pub job: String,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerRestartUnitReply {
    pub fn new(job: String, unit: crate::SystemdUnit) -> SystemdManagerRestartUnitReply {
        SystemdManagerRestartUnitReply {
            job,
            unit: Box::new(unit),
        }
    }
}
