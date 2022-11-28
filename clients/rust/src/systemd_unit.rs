// SystemdUnit represents a SystemdUnit model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdUnit {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename="fragment_path", skip_serializing_if = "Option::is_none")]
    fragment_path: Option<String>,
    #[serde(rename="active_state", skip_serializing_if = "Option::is_none")]
    active_state: Option<Box<crate::SystemdUnitActiveState>>,
    #[serde(rename="load_state", skip_serializing_if = "Option::is_none")]
    load_state: Option<Box<crate::SystemdUnitLoadState>>,
    #[serde(rename="unit_file_state", skip_serializing_if = "Option::is_none")]
    unit_file_state: Option<Box<crate::SystemdUnitFileState>>,
}

impl SystemdUnit {
    pub fn new(id: Option<String>, fragment_path: Option<String>, active_state: Option<crate::SystemdUnitActiveState>, load_state: Option<crate::SystemdUnitLoadState>, unit_file_state: Option<crate::SystemdUnitFileState>) -> SystemdUnit {
        SystemdUnit {
            id,
            fragment_path,
            active_state: active_state.map(Box::new),
            load_state: load_state.map(Box::new),
            unit_file_state: unit_file_state.map(Box::new),
        }
    }
}
