// SystemdManagerUnitJobReply represents a SystemdManagerUnitJobReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerUnitJobReply {
    #[serde(rename="job")]
    pub job: String,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerUnitJobReply {
    pub fn new(job: String, unit: crate::SystemdUnit) -> SystemdManagerUnitJobReply {
        SystemdManagerUnitJobReply {
            job,
            unit: Box::new(unit),
        }
    }
}
