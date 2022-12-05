// SystemdUnitActiveStateChanged represents a SystemdUnitActiveStateChanged model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemdUnitActiveStateChanged {
    #[serde(rename="active_state")]
    pub active_state: Box<crate::SystemdUnitActiveState>,
    #[serde(rename="unit")]
    pub unit: Box<crate::SystemdUnit>,
}

impl SystemdUnitActiveStateChanged {
    pub fn new(active_state: crate::SystemdUnitActiveState, unit: crate::SystemdUnit) -> SystemdUnitActiveStateChanged {
        SystemdUnitActiveStateChanged {
            active_state: Box::new(active_state),
            unit: Box::new(unit),
        }
    }
}
