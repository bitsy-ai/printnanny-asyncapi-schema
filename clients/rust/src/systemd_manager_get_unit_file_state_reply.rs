// SystemdManagerGetUnitFileStateReply represents a SystemdManagerGetUnitFileStateReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdManagerGetUnitFileStateReply {
    #[serde(rename="unit_file_state", skip_serializing_if = "Option::is_none")]
    pub unit_file_state: Option<Box<crate::SystemdUnitFileState>>,
}

impl SystemdManagerGetUnitFileStateReply {
    pub fn new(unit_file_state: Option<crate::SystemdUnitFileState>) -> SystemdManagerGetUnitFileStateReply {
        SystemdManagerGetUnitFileStateReply {
            unit_file_state: unit_file_state.map(Box::new),
        }
    }
}
