// SystemdUnitChange represents a SystemdUnitChange model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdUnitChange {
    #[serde(rename="change")]
    pub change: Box<crate::SystemdUnitChangeState>,
    #[serde(rename="file")]
    pub file: String,
    #[serde(rename="destination")]
    pub destination: String,
}

impl SystemdUnitChange {
    pub fn new(change: crate::SystemdUnitChangeState, file: String, destination: String) -> SystemdUnitChange {
        SystemdUnitChange {
            change: Box::new(change),
            file,
            destination,
        }
    }
}
