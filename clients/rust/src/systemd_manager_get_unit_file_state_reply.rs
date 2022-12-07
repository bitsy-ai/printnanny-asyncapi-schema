// SystemdManagerGetUnitFileStateReply represents a SystemdManagerGetUnitFileStateReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitFileStateReply {
    #[serde(rename="unit_file_state")]
    pub unit_file_state: Box<crate::SystemdUnitFileState>,
}

impl SystemdManagerGetUnitFileStateReply {
    pub fn new(unit_file_state: crate::SystemdUnitFileState) -> SystemdManagerGetUnitFileStateReply {
        SystemdManagerGetUnitFileStateReply {
            unit_file_state: Box::new(unit_file_state),
        }
    }
}
