// SystemdUnitFileStateChanged represents a SystemdUnitFileStateChanged model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdUnitFileStateChanged {
    #[serde(rename="unit_file_state")]
    pub unit_file_state: Box<crate::SystemdUnitFileState>,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdUnitFileStateChanged {
    pub fn new(unit_file_state: crate::SystemdUnitFileState, unit: crate::SystemdUnit) -> SystemdUnitFileStateChanged {
        SystemdUnitFileStateChanged {
            unit_file_state: Box::new(unit_file_state),
            unit: Box::new(unit),
        }
    }
}
