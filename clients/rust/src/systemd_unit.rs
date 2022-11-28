// SystemdUnit represents a SystemdUnit model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdUnit {
    #[serde(rename="id")]
    id: String,
    #[serde(rename="fragment_path")]
    fragment_path: String,
    #[serde(rename="active_state")]
    active_state: Box<crate::SystemdUnitActiveState>,
    #[serde(rename="load_state")]
    load_state: Box<crate::SystemdUnitLoadState>,
    #[serde(rename="unit_file_state")]
    unit_file_state: Box<crate::SystemdUnitFileState>,
}

impl SystemdUnit {
    pub fn new(id: String, fragment_path: String, active_state: crate::SystemdUnitActiveState, load_state: crate::SystemdUnitLoadState, unit_file_state: crate::SystemdUnitFileState) -> SystemdUnit {
        SystemdUnit {
            id,
            fragment_path,
            active_state: Box::new(active_state),
            load_state: Box::new(load_state),
            unit_file_state: Box::new(unit_file_state),
        }
    }
}
