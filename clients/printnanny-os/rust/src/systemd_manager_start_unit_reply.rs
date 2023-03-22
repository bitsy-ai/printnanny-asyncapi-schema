// SystemdManagerStartUnitReply represents a SystemdManagerStartUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerStartUnitReply {
    #[serde(rename="job")]
    pub job: String,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerStartUnitReply {
    pub fn new(job: String, unit: crate::SystemdUnit) -> SystemdManagerStartUnitReply {
        SystemdManagerStartUnitReply {
            job,
            unit: Box::new(unit),
        }
    }
}
