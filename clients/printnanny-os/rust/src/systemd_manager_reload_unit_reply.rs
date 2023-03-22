// SystemdManagerReloadUnitReply represents a SystemdManagerReloadUnitReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerReloadUnitReply {
    #[serde(rename="job")]
    pub job: String,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdManagerReloadUnitReply {
    pub fn new(job: String, unit: crate::SystemdUnit) -> SystemdManagerReloadUnitReply {
        SystemdManagerReloadUnitReply {
            job,
            unit: Box::new(unit),
        }
    }
}
